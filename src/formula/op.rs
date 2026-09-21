//! The binary operators of propositional logic, and their truth tables.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    And,
    Or,
    Xor,
    Imply,
    Equiv,
}

impl Op {
    pub fn from_char(c: char) -> Option<Op> {
        match c {
            '&' => Some(Op::And),
            '|' => Some(Op::Or),
            '^' => Some(Op::Xor),
            '>' => Some(Op::Imply),
            '=' => Some(Op::Equiv),
            _ => None,
        }
    }

    pub fn as_char(self) -> char {
        match self {
            Op::And => '&',
            Op::Or => '|',
            Op::Xor => '^',
            Op::Imply => '>',
            Op::Equiv => '=',
        }
    }

    pub fn apply(self, left: bool, right: bool) -> bool {
        match self {
            Op::And => left & right,
            Op::Or => left | right,
            Op::Xor => left ^ right,
            Op::Imply => !left | right,
            Op::Equiv => left == right,
        }
    }
}
