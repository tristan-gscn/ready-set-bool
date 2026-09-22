// Shared module: each exercise uses a subset of it.
#[allow(dead_code, unused_imports)]
#[path = "../formula/mod.rs"]
mod formula;

use formula::parse;

pub fn sat(formula: &str) -> bool {
    match parse(formula) {
        Ok(tree) => tree.is_satisfiable(),
        Err(reason) => {
            eprintln!("error: {formula:?}: {reason}");
            false
        }
    }
}

fn main() {
    for formula in ["AB|", "AB&", "AA!&", "AA^"] {
        println!("sat(\"{formula}\") = {}", sat(formula));
    }

    println!();
    // The truth table shows why: a formula is satisfiable when its column
    // holds at least one 1.
    for formula in ["AB>", "AA!&"] {
        println!("{formula}  -> sat = {}", sat(formula));
        if let Ok(tree) = parse(formula) {
            tree.for_each_row(|values, result| {
                let values = values
                    .iter()
                    .map(|v| if *v { "1" } else { "0" })
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("  {values} | {}", u8::from(result));
            });
        }
        println!();
    }

    println!("{}", sat("AB"));
}

#[cfg(test)]
mod tests {
    use super::formula::Op;
    use super::*;

    #[test]
    fn subject_examples() {
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
    }

    #[test]
    fn a_single_variable_is_satisfiable() {
        assert!(sat("A"));
        assert!(sat("A!"));
    }

    #[test]
    fn constants_decide_on_their_own() {
        assert!(sat("1"));
        assert!(!sat("0"));
        assert!(!sat("10&"));
        assert!(sat("10|"));
        assert!(!sat("A0&"));
        assert!(sat("A1|"));
    }

    #[test]
    fn contradictions_are_unsatisfiable() {
        assert!(!sat("AA!&"));
        assert!(!sat("AA^"));
        assert!(!sat("AA!="));
        assert!(!sat("AA!&BB!&|"));
        // (A | B) & !A & !B
        assert!(!sat("AB|A!&B!&"));
    }

    #[test]
    fn tautologies_are_satisfiable() {
        assert!(sat("AA!|"));
        assert!(sat("AA="));
        assert!(sat("AA>"));
        assert!(sat("AB>A>A>"));
    }

    #[test]
    fn satisfiable_by_a_single_row_only() {
        // A & B & C & D holds for exactly one of the sixteen rows.
        assert!(sat("AB&C&D&"));
        // ...and negating it makes that row the only false one.
        assert!(sat("AB&C&D&!"));
    }

    #[test]
    fn invalid_formulas_are_reported_as_unsatisfiable() {
        assert!(!sat(""));
        assert!(!sat("AB"));
        assert!(!sat("1a&"));
    }

    #[test]
    fn agrees_with_the_truth_table() {
        let mut rng: u64 = 0x2545F4914F6CDD1D;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };

        for _ in 0..200 {
            let formula = random_formula(&mut next, 3);
            let tree = parse(&formula).unwrap();

            // The definition, spelled out: at least one row comes out true.
            let mut any_row_is_true = false;
            tree.for_each_row(|_, result| any_row_is_true |= result);

            assert_eq!(tree.is_satisfiable(), any_row_is_true, "formula: {formula}");
        }
    }

    #[test]
    fn normal_forms_preserve_satisfiability() {
        let mut rng: u64 = 0x9E3779B97F4A7C15;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };

        for _ in 0..200 {
            let formula = random_formula(&mut next, 3);
            let tree = parse(&formula).unwrap();
            let expected = tree.is_satisfiable();

            assert_eq!(tree.nnf().is_satisfiable(), expected, "nnf of {formula}");
            assert_eq!(tree.cnf().is_satisfiable(), expected, "cnf of {formula}");
        }
    }

    /// A random well-formed formula over A..D, in reverse polish notation.
    fn random_formula(next: &mut impl FnMut() -> u64, depth: u32) -> String {
        if depth == 0 || next().is_multiple_of(4) {
            return ((b'A' + (next() % 4) as u8) as char).to_string();
        }

        match next() % 6 {
            0 => format!("{}!", random_formula(next, depth - 1)),
            n => {
                let op = [Op::And, Op::Or, Op::Xor, Op::Imply, Op::Equiv][n as usize - 1];
                format!(
                    "{}{}{}",
                    random_formula(next, depth - 1),
                    random_formula(next, depth - 1),
                    op.as_char()
                )
            }
        }
    }
}
