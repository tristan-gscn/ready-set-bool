//! Enumerating every assignment of a formula's variables.
//!
//! Shared by the truth table (ex04) and by SAT (ex07): both walk the same
//! 2^n rows, one only prints them while the other looks for a true one.

use super::tree::Formula;

impl Formula {
    /// The distinct variables of the formula, in alphabetical order.
    ///
    /// `AB&A|` holds two of them, `['A', 'B']`: a variable used several
    /// times is reported once.
    pub fn variables(&self) -> Vec<char> {
        // One slot per letter: marking is idempotent, and reading the slots
        // back in order yields the variables sorted and deduplicated.
        let mut seen = [false; 26];
        self.mark_variables(&mut seen);

        seen.iter()
            .enumerate()
            .filter(|(_, seen)| **seen)
            .map(|(i, _)| (b'A' + i as u8) as char)
            .collect()
    }

    fn mark_variables(&self, seen: &mut [bool; 26]) {
        match self {
            Formula::Value(_) => {}
            Formula::Var(name) => seen[*name as usize - 'A' as usize] = true,
            Formula::Not(inner) => inner.mark_variables(seen),
            Formula::Binary(_, left, right) => {
                left.mark_variables(seen);
                right.mark_variables(seen);
            }
        }
    }

    /// Calls `row` once per line of the truth table.
    ///
    /// Rows come in the order the subject asks for: the first variable of
    /// [`Formula::variables`] is the most significant bit, so the lines count
    /// up in binary from all-false to all-true.
    ///
    /// `row` receives the values of the variables — in the same order as
    /// [`Formula::variables`] — and what the formula evaluates to for them.
    /// A formula without variables still yields exactly one row.
    pub fn for_each_row(&self, mut row: impl FnMut(&[bool], bool)) {
        self.each_row_while(|values, result| {
            row(values, result);
            true
        });
    }

    /// Whether some assignment of the variables makes the formula true.
    ///
    /// Brute force over the truth table, which the subject allows: its
    /// maximum time complexity for ex07 is O(2^n). The walk stops at the
    /// first row that comes out true, so only an unsatisfiable formula
    /// actually costs the full 2^n.
    pub fn is_satisfiable(&self) -> bool {
        let mut satisfiable = false;

        self.each_row_while(|_, result| {
            satisfiable = result;
            !result // keep going as long as no row is true
        });

        satisfiable
    }

    /// Walks the assignments, stopping as soon as `row` returns false.
    ///
    /// Returns whether every row was visited, so a caller can tell a walk
    /// that ran to the end from one that broke out early.
    fn each_row_while(&self, mut row: impl FnMut(&[bool], bool) -> bool) -> bool {
        let variables = self.variables();
        let count = variables.len();

        // `assignment` is what eval reads, indexed by letter; `line` is the
        // same values in table order, which is what the caller wants.
        let mut assignment = [false; 26];
        let mut line = vec![false; count];

        // A formula without variables has 2^0 == 1 row.
        for i in 0..(1u64 << count) {
            for (j, name) in variables.iter().enumerate() {
                let value = (i >> (count - 1 - j)) & 1 == 1;
                line[j] = value;
                assignment[*name as usize - 'A' as usize] = value;
            }

            if !row(&line, self.eval(&assignment)) {
                return false;
            }
        }

        true
    }
}
