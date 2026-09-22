//! Conjunctive Normal Form: a conjunction of clauses, each one a disjunction
//! of literals.
//!
//! The rewriting goes through a list of clauses rather than through the tree.
//! In a CNF the nesting is fixed — and above, or below, literals at the
//! bottom — so `Vec<Vec<Formula>>` carries everything, and the associativity
//! the subject asks for is chosen once, when the tree is built back.

use super::op::Op;
use super::tree::Formula;

/// Folds the parts into a single tree, associating to the right.
///
/// `[A, B, C]` with `&` gives `A & (B & C)`, whose reverse polish notation is
/// `ABC&&` — every conjunction at the end of the formula, as ex06 requires.
fn fold_right(mut parts: Vec<Formula>, op: Op) -> Formula {
    let last = parts
        .pop()
        .expect("clauses() never yields an empty list, nor an empty clause");

    parts.into_iter().rev().fold(last, |acc, part| {
        Formula::Binary(op, Box::new(part), Box::new(acc))
    })
}

impl Formula {
    /// The formula in Conjunctive Normal Form, as ex06 asks for.
    pub fn cnf(&self) -> Formula {
        let clauses = self
            .nnf()
            .clauses()
            .into_iter()
            .map(|clause| fold_right(clause, Op::Or))
            .collect();

        fold_right(clauses, Op::And)
    }

    /// The clauses of a formula, distributed as it goes.
    ///
    /// | node | clauses |
    /// |---|---|
    /// | a literal | `[[it]]` |
    /// | `X & Y` | those of X, then those of Y |
    /// | `X \| Y` | every clause of X merged with every clause of Y |
    ///
    /// That last line is the distributivity law: `(c1 & c2) \| (c3 & c4)`
    /// equals `(c1\|c3) & (c1\|c4) & (c2\|c3) & (c2\|c4)`. Sizes multiply
    /// instead of adding, which is where the exponential growth the subject
    /// warns about comes from.
    ///
    /// Assumes the formula is already in negation normal form, so the only
    /// operators left are `&` and `|` and every `!` sits on a leaf.
    pub fn clauses(&self) -> Vec<Vec<Formula>> {
        match self {
            Formula::Binary(Op::And, left, right) => {
                let mut clauses = left.clauses();
                clauses.extend(right.clauses());
                clauses
            }

            Formula::Binary(Op::Or, left, right) => {
                let left = left.clauses();
                let right = right.clauses();
                let mut product = Vec::with_capacity(left.len() * right.len());

                for left_clause in &left {
                    for right_clause in &right {
                        let mut merged = left_clause.clone();
                        merged.extend(right_clause.iter().cloned());
                        product.push(merged);
                    }
                }

                product
            }

            // A literal — or an operator NNF should have removed, in which
            // case treating it as opaque keeps the meaning, only not the form.
            literal => vec![vec![literal.clone()]],
        }
    }

    /// Whether the formula is a conjunction of disjunctions of literals.
    ///
    /// This is the mathematical definition, which says nothing about
    /// associativity; the subject's stricter wording — every conjunction at
    /// the end of the formula — is checked on the printed output instead.
    pub fn is_cnf(&self) -> bool {
        match self {
            Formula::Binary(Op::And, left, right) => left.is_cnf() && right.is_cnf(),
            clause => clause.is_clause(),
        }
    }

    /// Whether the formula is a disjunction of literals.
    fn is_clause(&self) -> bool {
        match self {
            Formula::Binary(Op::Or, left, right) => left.is_clause() && right.is_clause(),
            literal => literal.is_literal(),
        }
    }

    /// Whether the formula is a variable or a constant, negated or not.
    fn is_literal(&self) -> bool {
        match self {
            Formula::Value(_) | Formula::Var(_) => true,
            Formula::Not(inner) => matches!(**inner, Formula::Value(_) | Formula::Var(_)),
            Formula::Binary(..) => false,
        }
    }
}
