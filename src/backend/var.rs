use llvm_ir::{constant::Float, name};

use super::gen_asm::GenTool;

#[derive(Clone, Debug)]
pub enum Var {
    Prim(PrimVar),
    Str(Str),
    Arr(ArrVar),
}

#[derive(Clone, Debug)]
pub enum PrimVar {
    IntVar(IntVar),
    FloatVar(FloatVar),
}
#[derive(Clone, Debug)]
pub struct IntVar {
    pub name: String,
    pub init: Option<i64>,
    pub is_const: bool,
}
#[derive(Clone, Debug)]
pub struct Str {
    pub name: String,
    pub init: Option<String>,
    pub is_const: bool,
}
impl Str {
    fn gen_asm(&self) -> String {
        String::new()
    }
}
#[derive(Clone, Debug)]
pub struct FloatVar {
    pub name: String,
    pub init: Option<f64>,
    pub is_const: bool,
}
#[derive(Clone, Debug)]
pub struct ArrVar {
    pub name: String,
    pub size: usize,
    pub init: Vec<PrimVar>,
    pub is_const: bool,
}

impl ArrVar {
    pub fn gen_asm(&self) -> String {
        // TODO
        String::new()
    }
}
impl PrimVar {
    pub fn gen_asm(&self) -> String {
        // match self {
        //     IntVar(i) => {
        //         let n=i.n
        //         // GenTool::gen_word(, val)
        //     }
        //     FloatVar(f) => {}
        // }
        // TODO
        String::new()
    }
}

impl Var {
    pub fn gen_asm(&self) -> String {
        match self {
            Var::Prim(prim) => prim.gen_asm(),
            Var::Str(str) => str.gen_asm(),
            Var::Arr(arr) => arr.gen_asm(),
        }
    }
}
