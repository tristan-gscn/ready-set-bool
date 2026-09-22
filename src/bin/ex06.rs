// Shared module: each exercise uses a subset of it.
#[allow(dead_code, unused_imports)]
#[path = "../formula/mod.rs"]
mod formula;

use formula::parse;

pub fn conjunctive_normal_form(formula: &str) -> String {
    match parse(formula) {
        Ok(tree) => tree.cnf().to_string(),
        Err(reason) => {
            eprintln!("error: {formula:?}: {reason}");
            String::new()
        }
    }
}

fn main() {
    for formula in [
        "AB&!", "AB|!", "AB|C&", "AB|C|D|", "AB&C&D&", "AB&!C!|", "AB|!C!&",
    ] {
        println!("{formula:<10} -> {}", conjunctive_normal_form(formula));
    }

    println!();
    let formula = "ABCD&|&";
    match parse(formula) {
        Ok(tree) => {
            println!("{formula}    A & (B | (C & D))");
            print!("{}", tree.tree());
            println!();

            let cnf = tree.cnf();
            println!("{cnf}  A & (B | C) & (B | D)");
            print!("{}", cnf.tree());
            println!();
            println!("clauses: {:?}", clause_strings(&tree.nnf()));
        }
        Err(reason) => eprintln!("error: {formula:?}: {reason}"),
    }

    println!();
    println!("{:?}", conjunctive_normal_form("AB"));
}

/// The clause list of a formula, as strings, for display only.
fn clause_strings(tree: &formula::Formula) -> Vec<Vec<String>> {
    tree.clauses()
        .iter()
        .map(|clause| clause.iter().map(|l| l.to_string()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::formula::{Formula, Op};
    use super::*;

    /// The whole truth table of a formula, as rows of (values, result).
    fn rows(tree: &Formula) -> Vec<(Vec<bool>, bool)> {
        let mut out = Vec::new();
        tree.for_each_row(|values, result| out.push((values.to_vec(), result)));
        out
    }

    /// The subject's own wording: "every conjunction must be located at the
    /// end of the formula". In reverse polish notation that means no symbol
    /// follows the first `&`.
    fn conjunctions_are_all_at_the_end(rpn: &str) -> bool {
        match rpn.find('&') {
            None => true,
            Some(first) => rpn[first..].chars().all(|c| c == '&'),
        }
    }

    /// Asserts everything the subject asks for at once: the result is a CNF,
    /// its conjunctions sit at the end, and it means the same thing.
    fn assert_is_a_valid_cnf_of(formula: &str) {
        let tree = parse(formula).unwrap();
        let cnf = tree.cnf();
        let printed = cnf.to_string();

        assert!(cnf.is_cnf(), "{formula} -> {printed} is not a CNF");
        assert!(
            conjunctions_are_all_at_the_end(&printed),
            "{formula} -> {printed} has a conjunction in the middle"
        );
        assert_eq!(
            rows(&tree),
            rows(&cnf),
            "{formula} -> {printed} changed the truth table"
        );
    }

    #[test]
    fn subject_examples() {
        assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
        assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
        assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
        assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
        assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
        assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
        assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
    }

    #[test]
    fn the_example_from_the_instructions() {
        // A & (B | (C & D))  ->  A & (B | C) & (B | D)
        assert_eq!(conjunctive_normal_form("ABCD&|&"), "ABC|BD|&&");
    }

    #[test]
    fn distributivity_is_applied() {
        // A | (B & C)  ->  (A | B) & (A | C)
        assert_eq!(conjunctive_normal_form("ABC&|"), "AB|AC|&");
        // (A & B) | C  ->  (A | C) & (B | C)
        assert_eq!(conjunctive_normal_form("AB&C|"), "AC|BC|&");
    }

    #[test]
    fn clauses_are_flattened_and_reassociated() {
        // Both nestings collapse to the same clause.
        assert_eq!(conjunctive_normal_form("AB|C|"), "ABC||");
        assert_eq!(conjunctive_normal_form("ABC||"), "ABC||");
        assert_eq!(conjunctive_normal_form("AB&C&"), "ABC&&");
        assert_eq!(conjunctive_normal_form("ABC&&"), "ABC&&");
    }

    #[test]
    fn the_clause_list_matches_the_formula() {
        let clauses = |formula: &str| {
            parse(formula)
                .unwrap()
                .nnf()
                .clauses()
                .iter()
                .map(|clause| clause.iter().map(|l| l.to_string()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        };

        assert_eq!(clauses("A"), vec![vec!["A"]]);
        assert_eq!(clauses("AB&"), vec![vec!["A"], vec!["B"]]);
        assert_eq!(clauses("AB|"), vec![vec!["A", "B"]]);
        // A & (B | (C & D))
        assert_eq!(
            clauses("ABCD&|&"),
            vec![vec!["A"], vec!["B", "C"], vec!["B", "D"]]
        );
    }

    #[test]
    fn every_operator_is_eliminated() {
        for formula in ["AB>", "AB=", "AB^", "AB>CD=^", "AB^C=DE>&"] {
            assert_is_a_valid_cnf_of(formula);
        }
    }

    #[test]
    fn negations_reach_the_leaves() {
        for formula in ["AB&!", "AB|!", "AB&CD|&!", "AB>!", "AB=!"] {
            assert_is_a_valid_cnf_of(formula);
        }
    }

    #[test]
    fn constants_survive_the_rewriting() {
        for formula in ["1", "10&", "A1>", "A0=!", "1AB&|"] {
            assert_is_a_valid_cnf_of(formula);
        }
    }

    #[test]
    fn a_formula_already_in_normal_form_is_left_alone() {
        for formula in ["A", "A!", "AB&", "AB|", "A!B!|", "AB|C&", "ABC||"] {
            assert_eq!(conjunctive_normal_form(formula), formula);
        }
    }

    #[test]
    fn invalid_formulas_return_an_empty_string() {
        assert_eq!(conjunctive_normal_form(""), "");
        assert_eq!(conjunctive_normal_form("AB"), "");
        assert_eq!(conjunctive_normal_form("1a&"), "");
    }

    #[test]
    fn holds_on_random_formulas() {
        let mut rng: u64 = 0x2545F4914F6CDD1D;
        let mut next = move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        };

        for _ in 0..200 {
            assert_is_a_valid_cnf_of(&random_formula(&mut next, 3));
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
