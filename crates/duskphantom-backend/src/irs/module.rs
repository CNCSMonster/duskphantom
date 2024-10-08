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

// 一个module 是一个 基础独立编译单元, 或者理解成c的一个 独立代码文件

use libthrd::LIB_THRD;

use super::*;
use crate::config::CONFIG;
pub struct Module {
    // module name
    pub name: String,
    // global var ,including primtype var and arr var
    pub global: Vec<var::Var>,
    // all funcs
    pub funcs: Vec<func::Func>,
    // entry func name
    pub entry: Option<String>,
}

impl Module {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Module {
            name: name.to_string(),
            global: vec![],
            funcs: vec![],
            entry: None,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn entry(&self) -> Option<&func::Func> {
        if let Some(entry) = self.entry.as_ref() {
            for func in self.funcs.iter() {
                if func.name() == entry.as_str() {
                    return Some(func);
                }
            }
        }
        None
    }
    pub fn gen_asm(&self) -> String {
        let mut global = String::new();
        if CONFIG.num_parallel_for_global_gen_asm <= 1 {
            println!("num_parallel_for_global_gen_asm <= 1");
            for v in self.global.iter() {
                global.push_str(v.gen_asm().as_str());
                global.push('\n');
            }
        } else {
            let thread_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(CONFIG.num_parallel_for_global_gen_asm)
                .build()
                .unwrap();
            global = thread_pool.install(|| {
                self.global
                    .par_iter()
                    .map(|v| v.gen_asm())
                    .collect::<Vec<String>>()
                    .join("\n")
            });
        }
        // sort funcs by name

        let mut fs: Vec<&func::Func> = self.funcs.iter().collect();
        fs.sort_by_cached_key(|f| f.name());
        let mut funcs = String::with_capacity(1024);
        if CONFIG.num_parallel_for_func_gen_asm <= 1 {
            println!("num_parallel_for_func_gen_asm <= 1");
            for f in fs.iter() {
                funcs.push_str(f.gen_asm().as_str());
                funcs.push('\n');
            }
        } else {
            let thread_pool = rayon::ThreadPoolBuilder::new()
                .num_threads(CONFIG.num_parallel_for_func_gen_asm)
                .build()
                .unwrap();
            funcs = thread_pool.install(|| {
                fs.par_iter()
                    .map(|f| f.gen_asm())
                    .collect::<Vec<String>>()
                    .join("\n")
            });
        };

        funcs.push_str(LIB_THRD);

        gen_asm::GenTool::gen_prog("test.c", global.as_str(), funcs.as_str())
    }
}
