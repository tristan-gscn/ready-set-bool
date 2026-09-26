//! Propositional formulas: from reverse polish notation, to a tree, to a value.

pub mod cnf;
pub mod nnf;
pub mod op;
pub mod parser;
pub mod sets;
pub mod table;
pub mod tree;

pub use op::Op;
pub use parser::{parse, ParseError};
pub use sets::SetEvalError;
pub use tree::Formula;
