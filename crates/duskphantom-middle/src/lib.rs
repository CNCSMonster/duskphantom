// Copyright 2024 Duskphantom Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use duskphantom_frontend as frontend;
use duskphantom_utils::mem::ObjPtr;
use ir::ir_builder::IRBuilder;
use transform::ultimate_pass;

pub mod analysis;
pub mod config;
pub mod errors;
pub mod ir;
pub mod irgen;
pub mod transform;

use std::pin::Pin;

pub struct Program {
    pub module: ir::Module,
    pub mem_pool: Pin<Box<IRBuilder>>,
}

use anyhow::{Context, Result};
use duskphantom_utils::context;

impl TryFrom<&frontend::Program> for Program {
    type Error = anyhow::Error;
    fn try_from(program: &frontend::Program) -> Result<Self> {
        irgen::gen(program).with_context(|| context!())
    }
}
impl TryFrom<frontend::Program> for Program {
    type Error = anyhow::Error;
    fn try_from(program: frontend::Program) -> Result<Self> {
        irgen::gen(&program)
    }
}

pub fn optimize(program: &mut Program, level: usize) {
    if level == 0 {
        return;
    }
    ultimate_pass::optimize_program(program, level).unwrap();
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    pub fn new() -> Self {
        let program_mem_pool = Box::pin(IRBuilder::new());
        let mem_pool: ObjPtr<IRBuilder> = ObjPtr::new(&program_mem_pool);
        Self {
            mem_pool: program_mem_pool,
            module: ir::Module::new(mem_pool),
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.mem_pool.clear();
    }
}
