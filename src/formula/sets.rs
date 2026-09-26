//! Evaluating a formula over sets instead of booleans.
//!
//! No second evaluator is needed. The subject makes the encompassing set the
//! union of the sets given, so an element belongs to the result exactly when
//! the boolean formula holds for it, each variable reading as "this element
//! is in my set". Sieving the union through [`Formula::eval`] is the whole
//! exercise: `&` sieves to an intersection, `|` to a union, `!` to a
//! complement, and `> = ^` come along for free.

use super::tree::Formula;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

/// The union of every set given, keeping the order elements first appear.
///
/// The subject calls it the globally encompassing set: outside of it, no
/// element could be named by any variable, so none can reach the result.
fn universe(sets: &[Vec<i32>]) -> Vec<i32> {
    let mut seen = HashSet::new();

    sets.iter()
        .flatten()
        .copied()
        .filter(|element| seen.insert(*element))
        .collect()
}

/// One boolean per letter: whether `element` belongs to that letter's set.
///
/// `A` reads the first set, `B` the second, and a letter past the end of the
/// list reads false — validation rejects that case before it can happen.
fn membership(element: i32, sets: &[HashSet<i32>]) -> [bool; 26] {
    std::array::from_fn(|i| sets.get(i).is_some_and(|set| set.contains(&element)))
}

impl Formula {
    /// The set the formula evaluates to, `A` being the first set given.
    ///
    /// The result holds no duplicates, and the subject states its order does
    /// not matter; elements come out in the order they first appear.
    pub fn eval_set(&self, sets: &[Vec<i32>]) -> Result<Vec<i32>, SetEvalError> {
        let variables = self.variables();

        if variables.len() != sets.len() {
            return Err(SetEvalError::CountMismatch {
                variables: variables.len(),
                sets: sets.len(),
            });
        }

        // The counts matching is not enough: "AC&" names two variables but
        // needs a third set for C, since letters index the list by position.
        if let Some(name) = variables
            .iter()
            .find(|name| **name as usize - 'A' as usize >= sets.len())
        {
            return Err(SetEvalError::NoSetFor(*name));
        }

        let members: Vec<HashSet<i32>> = sets
            .iter()
            .map(|set| set.iter().copied().collect())
            .collect();

        Ok(universe(sets)
            .into_iter()
            .filter(|element| self.eval(&membership(*element, &members)))
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetEvalError {
    /// The subject's own check: as many sets as the formula has variables.
    CountMismatch { variables: usize, sets: usize },
    /// A variable whose position falls past the end of the list.
    NoSetFor(char),
}

impl fmt::Display for SetEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SetEvalError::CountMismatch { variables, sets } => write!(
                f,
                "the formula holds {variables} variables but {sets} sets were given"
            ),
            SetEvalError::NoSetFor(name) => write!(f, "no set is given for '{name}'"),
        }
    }
}

impl Error for SetEvalError {}
