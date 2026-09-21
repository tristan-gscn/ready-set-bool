//! Reverse polish notation to formula tree.

use super::op::Op;
use super::tree::Formula;
use std::error::Error;
use std::fmt;

/// Builds the tree from a formula in reverse polish notation, in O(n).
///
/// Symbols: `0` and `1` for the constants, `!` for the negation, and
/// `& | ^ > =` for the binary operators. Anything else is invalid.
pub fn parse(formula: &str) -> Result<Formula, ParseError> {
    let mut stack: Vec<Formula> = Vec::new();

    for c in formula.chars() {
        match c {
            '0' => stack.push(Formula::Value(false)),
            '1' => stack.push(Formula::Value(true)),
            '!' => {
                let operand = stack.pop().ok_or(ParseError::MissingOperand(c))?;
                stack.push(Formula::Not(Box::new(operand)));
            }
            _ => {
                let op = Op::from_char(c).ok_or(ParseError::UnknownSymbol(c))?;
                // The right operand was pushed last, so it comes out first.
                let right = stack.pop().ok_or(ParseError::MissingOperand(c))?;
                let left = stack.pop().ok_or(ParseError::MissingOperand(c))?;
                stack.push(Formula::Binary(op, Box::new(left), Box::new(right)));
            }
        }
    }

    match stack.len() {
        0 => Err(ParseError::Empty),
        1 => Ok(stack.pop().unwrap()),
        n => Err(ParseError::MissingOperator(n)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    UnknownSymbol(char),
    MissingOperand(char),
    MissingOperator(usize),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "the formula is empty"),
            ParseError::UnknownSymbol(c) => write!(f, "unknown symbol '{c}'"),
            ParseError::MissingOperand(c) => write!(f, "'{c}' is missing an operand"),
            ParseError::MissingOperator(n) => {
                write!(
                    f,
                    "{n} operands are left unconnected, an operator is missing"
                )
            }
        }
    }
}

impl Error for ParseError {}
