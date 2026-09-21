//! The formula tree: its shape, its value, and the two ways it prints itself.

use super::op::Op;
use std::fmt;

/// A propositional formula, as a tree whose arity is fixed by the type:
/// `Not` always has one child, `Binary` always has two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    Value(bool),
    Not(Box<Formula>),
    Binary(Op, Box<Formula>, Box<Formula>),
}

impl Formula {
    /// Walks the tree and computes its value.
    pub fn eval(&self) -> bool {
        match self {
            Formula::Value(b) => *b,
            Formula::Not(inner) => !inner.eval(),
            Formula::Binary(op, left, right) => op.apply(left.eval(), right.eval()),
        }
    }

    /// Renders the tree, as suggested by the subject:
    ///
    /// ```text
    /// &
    /// ├── 1
    /// └── |
    ///     ├── 0
    ///     └── 1
    /// ```
    pub fn tree(&self) -> String {
        let mut out = self.label();
        out.push('\n');
        self.render_children("", &mut out);
        out
    }

    fn label(&self) -> String {
        match self {
            Formula::Value(false) => '0'.to_string(),
            Formula::Value(true) => '1'.to_string(),
            Formula::Not(_) => '!'.to_string(),
            Formula::Binary(op, _, _) => op.as_char().to_string(),
        }
    }

    fn children(&self) -> Vec<&Formula> {
        match self {
            Formula::Value(_) => Vec::new(),
            Formula::Not(inner) => vec![inner],
            Formula::Binary(_, left, right) => vec![left, right],
        }
    }

    fn render_children(&self, prefix: &str, out: &mut String) {
        let children = self.children();
        for (i, child) in children.iter().enumerate() {
            let (branch, extension) = if i + 1 == children.len() {
                ("└── ", "    ")
            } else {
                ("├── ", "│   ")
            };
            out.push_str(prefix);
            out.push_str(branch);
            out.push_str(&child.label());
            out.push('\n');
            child.render_children(&format!("{prefix}{extension}"), out);
        }
    }
}

// Writes the formula back in reverse polish notation.
impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Value(false) => write!(f, "0"),
            Formula::Value(true) => write!(f, "1"),
            Formula::Not(inner) => write!(f, "{inner}!"),
            Formula::Binary(op, left, right) => write!(f, "{left}{right}{}", op.as_char()),
        }
    }
}
