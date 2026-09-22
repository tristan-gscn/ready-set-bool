//! Negation Normal Form: only `!`, `&` and `|`, with every `!` on a leaf.
//!
//! The rewriting runs in two passes, in this order. Pushing negations first
//! would need De Morgan rules for `>` and `=` as well; getting rid of them
//! beforehand leaves only four cases to handle.

use super::op::Op;
use super::tree::Formula;

// Node constructors, so the rewrite rules below read like the rules they
// come from instead of drowning in Box::new.

fn not(inner: Formula) -> Formula {
    Formula::Not(Box::new(inner))
}

fn not_boxed(inner: Box<Formula>) -> Formula {
    Formula::Not(inner)
}

fn and(left: Formula, right: Formula) -> Formula {
    Formula::Binary(Op::And, Box::new(left), Box::new(right))
}

fn or(left: Formula, right: Formula) -> Formula {
    Formula::Binary(Op::Or, Box::new(left), Box::new(right))
}

impl Formula {
    /// The formula in Negation Normal Form, as ex05 asks for.
    pub fn nnf(&self) -> Formula {
        self.with_basic_operators().with_negations_pushed()
    }

    /// Rewrites `>`, `=` and `^` away, leaving only `!`, `&` and `|`.
    ///
    /// | rule | |
    /// |---|---|
    /// | `A > B` | `!A \| B` |
    /// | `A = B` | `(A & B) \| (!A & !B)` |
    /// | `A ^ B` | `(A & !B) \| (!A & B)` |
    ///
    /// Rewrite the children first: the tree this produces is bigger than the
    /// original — `=` and `^` duplicate both of their children — so recursing
    /// on the original, strictly smaller, children is what makes it end.
    pub fn with_basic_operators(&self) -> Formula {
        match self {
            Formula::Value(_) | Formula::Var(_) => self.clone(),
            Formula::Not(inner) => not(inner.with_basic_operators()),
            Formula::Binary(op, left, right) => {
                let left = left.with_basic_operators();
                let right = right.with_basic_operators();

                match op {
                    Op::And | Op::Or => Formula::Binary(*op, Box::new(left), Box::new(right)),
                    // A > B  ->  !A | B
                    Op::Imply => or(not(left), right),
                    // A = B  ->  (A & B) | (!A & !B)
                    Op::Equiv => or(and(left.clone(), right.clone()), and(not(left), not(right))),
                    // A ^ B  ->  (A & !B) | (!A & B)
                    Op::Xor => or(and(left.clone(), not(right.clone())), and(not(left), right)),
                }
            }
        }
    }

    /// Pushes every negation down to the leaves.
    ///
    /// | rule | |
    /// |---|---|
    /// | `!!A` | `A` |
    /// | `!(A & B)` | `!A \| !B` |
    /// | `!(A \| B)` | `!A & !B` |
    /// | `!A`, A a leaf | already normal, stop |
    ///
    /// Assumes [`Formula::with_basic_operators`] already ran, so the only
    /// operators left are `&` and `|`.
    pub fn with_negations_pushed(&self) -> Formula {
        match self {
            Formula::Value(_) | Formula::Var(_) => self.clone(),

            Formula::Not(inner) => match inner.as_ref() {
                // !!A  ->  A
                Formula::Not(twice) => twice.with_negations_pushed(),

                // !(A & B)  ->  !A | !B     and     !(A | B)  ->  !A & !B
                Formula::Binary(op, left, right) => {
                    let left = not_boxed(left.clone()).with_negations_pushed();
                    let right = not_boxed(right.clone()).with_negations_pushed();

                    match op {
                        Op::And => or(left, right),
                        Op::Or => and(left, right),
                        _ => unreachable!(
                            "with_basic_operators should have removed '{}' first",
                            op.as_char()
                        ),
                    }
                }

                // The negation already sits on a leaf.
                Formula::Value(_) | Formula::Var(_) => self.clone(),
            },

            Formula::Binary(op, left, right) => Formula::Binary(
                *op,
                Box::new(left.with_negations_pushed()),
                Box::new(right.with_negations_pushed()),
            ),
        }
    }

    /// Whether the formula is in Negation Normal Form: no operator besides
    /// `&` and `|`, and every negation sitting on a leaf.
    pub fn is_nnf(&self) -> bool {
        match self {
            Formula::Value(_) | Formula::Var(_) => true,
            Formula::Not(inner) => matches!(**inner, Formula::Value(_) | Formula::Var(_)),
            Formula::Binary(op, left, right) => {
                matches!(op, Op::And | Op::Or) && left.is_nnf() && right.is_nnf()
            }
        }
    }
}
