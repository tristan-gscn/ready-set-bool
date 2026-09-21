//! Propositional formulas: from reverse polish notation, to a tree, to a value.

pub mod op;
pub mod parser;
pub mod tree;

pub use op::Op;
pub use parser::{parse, ParseError};
pub use tree::Formula;
