// Shared module: each exercise uses a subset of it.
#[allow(dead_code, unused_imports)]
#[path = "../formula/mod.rs"]
mod formula;

use formula::parse;

pub fn negation_normal_form(formula: &str) -> String {
    match parse(formula) {
        Ok(tree) => tree.nnf().to_string(),
        Err(reason) => {
            eprintln!("error: {formula:?}: {reason}");
            String::new()
        }
    }
}

fn main() {
    for formula in ["AB&!", "AB|!", "AB>", "AB=", "AB|C&!"] {
        println!("{formula:<8} -> {}", negation_normal_form(formula));
    }

    println!();
    let formula = "AB|C&!";
    match parse(formula) {
        Ok(tree) => {
            println!("{formula}");
            print!("{}", tree.tree());
            println!();
            let nnf = tree.nnf();
            println!("{nnf}");
            print!("{}", nnf.tree());
        }
        Err(reason) => eprintln!("error: {formula:?}: {reason}"),
    }

    println!();
    println!("{:?}", negation_normal_form("AB"));
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

    /// Asserts the two properties the subject asks for at once: the result is
    /// in normal form, and it is equivalent to what went in.
    fn assert_is_a_valid_nnf_of(formula: &str) {
        let tree = parse(formula).unwrap();
        let nnf = tree.nnf();

        assert!(nnf.is_nnf(), "{formula} -> {nnf} is not in normal form");
        assert_eq!(
            rows(&tree),
            rows(&nnf),
            "{formula} -> {nnf} changed the truth table"
        );
    }

    #[test]
    fn subject_examples() {
        assert_eq!(negation_normal_form("AB&!"), "A!B!|");
        assert_eq!(negation_normal_form("AB|!"), "A!B!&");
        assert_eq!(negation_normal_form("AB>"), "A!B|");
        assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
        assert_eq!(negation_normal_form("AB|C&!"), "A!B!&C!|");
    }

    #[test]
    fn a_formula_already_in_normal_form_is_left_alone() {
        for formula in ["A", "A!", "AB&", "AB|", "A!B!|", "AB&C!|"] {
            assert_eq!(negation_normal_form(formula), formula);
        }
    }

    #[test]
    fn double_negations_are_removed() {
        assert_eq!(negation_normal_form("A!!"), "A");
        assert_eq!(negation_normal_form("A!!!"), "A!");
        assert_eq!(negation_normal_form("A!!!!"), "A");
    }

    #[test]
    fn de_morgan_applies_under_a_negation() {
        assert_is_a_valid_nnf_of("AB&!");
        assert_is_a_valid_nnf_of("AB|!");
        assert_is_a_valid_nnf_of("AB&C|!");
        assert_is_a_valid_nnf_of("AB&CD|&!");
    }

    #[test]
    fn material_conditions_are_eliminated() {
        assert_is_a_valid_nnf_of("AB>");
        assert_is_a_valid_nnf_of("AB>!");
        assert_is_a_valid_nnf_of("AB>C>");
        assert_is_a_valid_nnf_of("AB>CD>&");
    }

    #[test]
    fn equivalences_are_eliminated() {
        assert_is_a_valid_nnf_of("AB=");
        assert_is_a_valid_nnf_of("AB=!");
        assert_is_a_valid_nnf_of("AB=C=");
    }

    #[test]
    fn exclusive_disjunctions_are_eliminated() {
        assert_is_a_valid_nnf_of("AB^");
        assert_is_a_valid_nnf_of("AB^!");
        assert_is_a_valid_nnf_of("AB^C^");
    }

    #[test]
    fn constants_survive_the_rewriting() {
        assert_is_a_valid_nnf_of("1");
        assert_is_a_valid_nnf_of("10&");
        assert_is_a_valid_nnf_of("A1>");
        assert_is_a_valid_nnf_of("A0=!");
    }

    #[test]
    fn invalid_formulas_return_an_empty_string() {
        assert_eq!(negation_normal_form(""), "");
        assert_eq!(negation_normal_form("AB"), "");
        assert_eq!(negation_normal_form("1a&"), "");
    }

    #[test]
    fn each_pass_is_correct_on_its_own() {
        // Pass 1 keeps the meaning and leaves only !, & and |.
        for formula in ["AB>", "AB=", "AB^", "AB>CD=^"] {
            let tree = parse(formula).unwrap();
            let basic = tree.with_basic_operators();

            assert_eq!(rows(&tree), rows(&basic), "{formula} -> {basic}");
            assert!(
                only_basic_operators(&basic),
                "{formula} -> {basic} still holds >, = or ^"
            );
        }

        // Pass 2 keeps the meaning and reaches normal form, given pass 1 ran.
        for formula in ["AB&!", "AB|!", "A!!", "AB&CD|&!"] {
            let tree = parse(formula).unwrap().with_basic_operators();
            let pushed = tree.with_negations_pushed();

            assert_eq!(rows(&tree), rows(&pushed), "{formula} -> {pushed}");
            assert!(pushed.is_nnf(), "{formula} -> {pushed}");
        }
    }

    fn only_basic_operators(tree: &Formula) -> bool {
        match tree {
            Formula::Value(_) | Formula::Var(_) => true,
            Formula::Not(inner) => only_basic_operators(inner),
            Formula::Binary(op, left, right) => {
                matches!(op, Op::And | Op::Or)
                    && only_basic_operators(left)
                    && only_basic_operators(right)
            }
        }
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
            assert_is_a_valid_nnf_of(&random_formula(&mut next, 3));
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
