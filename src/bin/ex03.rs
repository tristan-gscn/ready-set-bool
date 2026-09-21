// Shared module: each exercise uses a subset of it.
#[allow(dead_code, unused_imports)]
#[path = "../formula/mod.rs"]
mod formula;

use formula::parse;

pub fn eval_formula(formula: &str) -> bool {
    match parse(formula) {
        Ok(tree) => tree.eval(),
        Err(reason) => {
            eprintln!("error: {formula:?}: {reason}");
            false
        }
    }
}

/// Prints the tree a formula parses into, as suggested by the subject.
fn show_tree(formula: &str) {
    match parse(formula) {
        Ok(tree) => {
            println!("{formula} = {}", tree.eval());
            println!();
            print!("{}", tree.tree());
        }
        Err(reason) => eprintln!("error: {formula:?}: {reason}"),
    }
}

fn main() {
    for formula in ["10&", "10|", "11>", "10=", "1011||="] {
        println!("eval_formula(\"{formula}\") = {}", eval_formula(formula));
    }

    println!();
    show_tree("101|&");
    println!();
    show_tree("1!01^>");

    println!();
    for formula in ["", "1&", "10", "1a&", "A1&"] {
        println!("eval_formula(\"{formula}\") = {}", eval_formula(formula));
    }
}

#[cfg(test)]
mod tests {
    use super::formula::{Formula, Op, ParseError};
    use super::*;

    #[test]
    fn subject_examples() {
        assert!(!eval_formula("10&"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11>"));
        assert!(!eval_formula("10="));
        assert!(eval_formula("1011||="));
    }

    #[test]
    fn constants() {
        assert!(eval_formula("1"));
        assert!(!eval_formula("0"));
    }

    #[test]
    fn negation() {
        assert!(eval_formula("0!"));
        assert!(!eval_formula("1!"));
        assert!(!eval_formula("0!!"));
    }

    #[test]
    fn conjunction() {
        assert!(!eval_formula("00&"));
        assert!(!eval_formula("01&"));
        assert!(!eval_formula("10&"));
        assert!(eval_formula("11&"));
    }

    #[test]
    fn disjunction() {
        assert!(!eval_formula("00|"));
        assert!(eval_formula("01|"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11|"));
    }

    #[test]
    fn exclusive_disjunction() {
        assert!(!eval_formula("00^"));
        assert!(eval_formula("01^"));
        assert!(eval_formula("10^"));
        assert!(!eval_formula("11^"));
    }

    #[test]
    fn material_condition_is_not_commutative() {
        assert!(eval_formula("00>"));
        assert!(eval_formula("01>"));
        assert!(!eval_formula("10>"));
        assert!(eval_formula("11>"));
    }

    #[test]
    fn logical_equivalence() {
        assert!(eval_formula("00="));
        assert!(!eval_formula("01="));
        assert!(!eval_formula("10="));
        assert!(eval_formula("11="));
    }

    #[test]
    fn grouping_changes_the_result() {
        // Same symbols in the same order, only the shape of the tree differs.
        // 0 > (1 > 0) holds, while (0 > 1) > 0 does not.
        assert!(eval_formula("010>>"));
        assert!(!eval_formula("01>0>"));

        // 0 & (0 | 1) is false, while (0 & 0) | 1 is true.
        assert!(!eval_formula("001|&"));
        assert!(eval_formula("00&1|"));
    }

    #[test]
    fn nested_formula() {
        // ((1 ^ 0) & (1 > 0)) | 1!! == (true & false) | true == true
        assert!(eval_formula("10^10>&1!!|"));
    }

    // The two tests below build the tree by hand, so they do not go through
    // parse. The printing one is already green: it only relies on what is
    // written, and is the baseline to check the tree is well formed.

    #[test]
    fn a_hand_built_tree_prints() {
        // 1 & (0 | 1)
        let tree = Formula::Binary(
            Op::And,
            Box::new(Formula::Value(true)),
            Box::new(Formula::Binary(
                Op::Or,
                Box::new(Formula::Value(false)),
                Box::new(Formula::Value(true)),
            )),
        );

        assert_eq!(tree.to_string(), "101|&");
        assert_eq!(tree.tree(), "&\n├── 1\n└── |\n    ├── 0\n    └── 1\n");
    }

    #[test]
    fn a_hand_built_tree_evaluates() {
        // 1 & (0 | 1)
        let tree = Formula::Binary(
            Op::And,
            Box::new(Formula::Value(true)),
            Box::new(Formula::Binary(
                Op::Or,
                Box::new(Formula::Value(false)),
                Box::new(Formula::Value(true)),
            )),
        );

        assert!(tree.eval());
    }

    #[test]
    fn parse_builds_the_expected_tree() {
        let expected = Formula::Binary(
            Op::And,
            Box::new(Formula::Value(true)),
            Box::new(Formula::Binary(
                Op::Or,
                Box::new(Formula::Value(false)),
                Box::new(Formula::Value(true)),
            )),
        );
        assert_eq!(parse("101|&"), Ok(expected));
    }

    #[test]
    fn display_round_trips_through_the_tree() {
        for formula in ["1", "0!", "10&", "101|&", "10^10>&1!!|", "110>>"] {
            assert_eq!(parse(formula).unwrap().to_string(), formula);
        }
    }

    #[test]
    fn rejects_invalid_formulas() {
        assert_eq!(parse(""), Err(ParseError::Empty));
        assert_eq!(parse("1a&"), Err(ParseError::UnknownSymbol('a')));
        assert_eq!(parse("1 0&"), Err(ParseError::UnknownSymbol(' ')));
        // Letters are variables from ex04 on, but not valid input here.
        assert_eq!(parse("A"), Err(ParseError::UnknownSymbol('A')));
        assert_eq!(parse("AB>"), Err(ParseError::UnknownSymbol('A')));
        assert_eq!(parse("&"), Err(ParseError::MissingOperand('&')));
        assert_eq!(parse("1&"), Err(ParseError::MissingOperand('&')));
        assert_eq!(parse("!"), Err(ParseError::MissingOperand('!')));
        assert_eq!(parse("10"), Err(ParseError::MissingOperator(2)));
        assert_eq!(parse("10&1"), Err(ParseError::MissingOperator(2)));
    }

    #[test]
    fn invalid_formulas_do_not_panic() {
        for formula in ["", "&", "!", "10", "1a&", "A"] {
            let _ = eval_formula(formula);
        }
    }

    #[test]
    fn matches_a_reference_evaluation_on_random_formulas() {
        let mut rng: u64 = 0x2545F4914F6CDD1D;
        let mut next = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };

        for _ in 0..10_000 {
            // Grow a random formula and its expected value side by side.
            let mut formula = String::new();
            let mut expected = next() & 1 == 1;
            formula.push(if expected { '1' } else { '0' });

            for _ in 0..(next() % 16) {
                match next() % 6 {
                    0 => {
                        formula.push('!');
                        expected = !expected;
                    }
                    n => {
                        let right = next() & 1 == 1;
                        let op = [Op::And, Op::Or, Op::Xor, Op::Imply, Op::Equiv][n as usize - 1];
                        formula.push(if right { '1' } else { '0' });
                        formula.push(op.as_char());
                        expected = op.apply(expected, right);
                    }
                }
            }

            assert_eq!(eval_formula(&formula), expected, "formula: {formula}");
        }
    }
}
