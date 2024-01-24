use self::program_mem_pool::ProgramMemPool;

use super::*;

/// 中间代码形式，由Moudle组织
pub struct Module {
    /// 全局变量集合，存放于基本块中，便于操作
    pub global_variables: BBPtr,
    pub mem_pool: ObjPtr<ProgramMemPool>,
    /// functions是函数集合，需要保证下标为0时为main函数，其余的位置可以随意安排
    pub functions: Vec<FunPtr>,
}

impl Module {
    /// 构造一个空的Module
    pub fn new(mem_pool: ObjPtr<ProgramMemPool>) -> Self {
        let golbal_variables = mem_pool
            .as_mut()
            .alloc_basic_block(BasicBlock::new("global".to_string()));
        Self {
            functions: Vec::new(),
            mem_pool,
            global_variables: mem_pool
                .as_mut()
                .alloc_basic_block(BasicBlock::new("global".to_string())),
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        // 释放内存池
        mem_pool::pool_clear();
    }
}
