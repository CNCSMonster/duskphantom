use anyhow::Result;

use crate::{
    backend::from_self::downcast_ref,
    middle::{
        analysis::memory_ssa::{MemorySSA, Node},
        ir::{
            instruction::{misc_inst::Call, InstType},
            Constant, InstPtr, Operand,
        },
        Program,
    },
};

pub fn optimize_program<'a>(program: &'a mut Program, memory_ssa: &'a mut MemorySSA) -> Result<()> {
    LoadElim::new(program, memory_ssa).run();
    Ok(())
}

struct LoadElim<'a, 'b> {
    program: &'a mut Program,
    memory_ssa: &'a mut MemorySSA<'b>,
}

impl<'a, 'b> LoadElim<'a, 'b> {
    fn new(program: &'a mut Program, memory_ssa: &'a mut MemorySSA<'b>) -> Self {
        Self {
            program,
            memory_ssa,
        }
    }

    fn run(&mut self) {
        for func in self.program.module.functions.clone().iter() {
            if func.is_lib() {
                continue;
            }
            for bb in func.rpo_iter() {
                for inst in bb.iter() {
                    self.process_inst(inst, func.is_main());
                }
            }
        }
    }

    fn process_inst(&mut self, mut load_inst: InstPtr, is_main: bool) {
        // Instruction must be load (instead of function call), otherwise it can't be optimized
        if load_inst.get_type() != InstType::Load {
            return;
        }

        // Get corresponding MemorySSA node
        let Some(load_node) = self.memory_ssa.get_inst_node(load_inst) else {
            return;
        };

        // It should be a predictable normal node (not entry or phi)
        // - when `a[1] = 3`, `load a[x]` is not predictable
        //   - because `x` may or may not be `1`
        // - `load a[1]` is predictable
        // - when `int a[3] = {}` (memset), `load a[x]` is predictable
        let Node::Normal(_, used_node, _, _, true) = load_node.as_ref() else {
            return;
        };

        // MemoryUse should use some node
        let Some(store_node) = used_node else {
            return;
        };

        // In main function if read from entry, load from global variable initializer
        if is_main {
            if let Node::Entry(_) = store_node.as_ref() {
                let load_op = load_inst.get_operand().first().unwrap();
                if let Some(op) = readonly_deref(load_op, vec![]) {
                    load_inst.replace_self(&op);
                    self.memory_ssa.remove_node(load_node);
                    return;
                }
            }
        }

        // The node used by MemoryUse should be a MemoryDef
        let Node::Normal(_, _, _, def_inst, _) = store_node.as_ref() else {
            return;
        };

        // If MemoryDef is store, replace with store operand
        if def_inst.get_type() == InstType::Store {
            let store_op = def_inst.get_operand().first().unwrap();
            load_inst.replace_self(store_op);
            self.memory_ssa.remove_node(load_node);
        }

        // If MemoryDef is memset, replace with constant
        // (we assume this memset sets 0 and is large enough)
        if def_inst.get_type() == InstType::Call {
            let call = downcast_ref::<Call>(def_inst.as_ref().as_ref());
            if call.func.is_memset() {
                let memset_op = &Operand::Constant(0.into());
                load_inst.replace_self(memset_op);
                self.memory_ssa.remove_node(load_node);
            }
        }
    }
}

fn readonly_deref<'a>(op: &'a Operand, mut index: Vec<&'a Operand>) -> Option<Operand> {
    match op {
        Operand::Global(gvar) => {
            let mut val = &gvar.initializer;

            // For a[0][1][2], it translates to something like `gep (gep a, _, 0), _, 1, 2`
            // Calling `readonly_deref` on it will first push `2, 1` to index array, and then push `0` (reversed!)
            // To get the final value, we iterate the index array in reverse order
            for i in index.iter().rev() {
                if let Constant::Array(arr) = val {
                    if let Operand::Constant(Constant::Int(i)) = i {
                        if let Some(element) = arr.get(*i as usize) {
                            val = element;
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                } else if let Constant::Zero(_) = val {
                    return Some(Operand::Constant(0.into()));
                } else {
                    return None;
                }
            }
            Some(val.clone().into())
        }
        Operand::Instruction(inst) => {
            if inst.get_type() == InstType::GetElementPtr {
                index.extend(inst.get_operand().iter().skip(2).rev());
                return readonly_deref(inst.get_operand().first().unwrap(), index);
            }
            None
        }
        _ => None,
    }
}
