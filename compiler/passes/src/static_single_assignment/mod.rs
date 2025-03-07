// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! The Static Single Assignment pass traverses the AST and converts it into SSA form.
//! See https://en.wikipedia.org/wiki/Static_single-assignment_form for more information.
//! The pass also replaces `DefinitionStatement`s with `AssignmentStatement`s.
//! The pass also simplifies complex expressions into a sequence of `AssignStatement`s. For example, `(a + b) * c` is rewritten into `$var$1 = a + b; $var$2 = $var$1 * c`.
//!
//! Consider the following Leo code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     if (flag == 0u8) {
//!         value += 1u8;
//!         return value;
//!     } else {
//!         value += 2u8;
//!     }
//!     return value;
//! }
//! ```
//!
//! The SSA pass produces the following code.
//! ```leo
//! function main(flag: u8, value: u8) -> u8 {
//!     $var$0 = flag == 0u8;
//!     if ($var$0) {
//!         $var$1 = value + 1u8;
//!         value$1 = $var$1;
//!         return value$1;
//!     } else {
//!         $var$2 = value + 2u8;
//!         value$2 = $var$2;
//!     }
//!     value$3 = $var$0 ? value$1 : value$2;
//!     return value$3;
//! }
//! ```
//! Note that the redundant assignments have no effect on the bytecode generated by the compiler.

mod rename_expression;

mod rename_program;

mod rename_statement;

pub mod static_single_assigner;
pub use static_single_assigner::*;

use crate::{Assigner, Pass, SymbolTable};

use leo_ast::{Ast, ProgramConsumer};
use leo_errors::Result;

impl<'a> Pass for StaticSingleAssigner<'a> {
    type Input = (Ast, &'a SymbolTable);
    type Output = Result<(Ast, Assigner)>;

    fn do_pass((ast, symbol_table): Self::Input) -> Self::Output {
        let mut consumer = StaticSingleAssigner::new(symbol_table);
        let program = consumer.consume_program(ast.into_repr());

        Ok((Ast::new(program), consumer.assigner))
    }
}
