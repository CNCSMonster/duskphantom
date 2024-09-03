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

mod block_fuse;
mod constant_fold;
mod dead_code_elim;
mod func_inline;
mod load_elim;
mod loop_optimization;
mod make_parallel;
mod mem2reg;
mod redundance_elim;
mod store_elim;
mod symbolic_eval;

pub use super::compiler;
pub use duskphantom_utils::diff::diff;
