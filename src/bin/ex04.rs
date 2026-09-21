// Shared module: each exercise uses a subset of it.
#[allow(dead_code, unused_imports)]
#[path = "../formula/mod.rs"]
mod formula;

use formula::{parse, ParseError};

pub fn print_truth_table(formula: &str) {
    match truth_table(formula) {
        Ok(table) => print!("{table}"),
        Err(reason) => eprintln!("error: {formula:?}: {reason}"),
    }
}

/// The truth table of a formula, in the layout the subject asks for:
///
/// ```text
/// | A | B | C | = |
/// |---|---|---|---|
/// | 0 | 0 | 0 | 0 |
/// ```
///
/// Kept apart from [`print_truth_table`] so the tests can read it back.
fn truth_table(formula: &str) -> Result<String, ParseError> {
    let tree = parse(formula)?;
    let variables = tree.variables();
    let mut out = String::new();

    out.push('|');
    for name in &variables {
        out.push_str(&format!(" {name} |"));
    }
    out.push_str(" = |\n");

    out.push('|');
    for _ in 0..variables.len() + 1 {
        out.push_str("---|");
    }
    out.push('\n');

    tree.for_each_row(|values, result| {
        out.push('|');
        for value in values {
            out.push_str(&format!(" {} |", u8::from(*value)));
        }
        out.push_str(&format!(" {} |\n", u8::from(result)));
    });

    Ok(out)
}

fn main() {
    // The example from the subject: (A & B) | C.
    let formula = "AB&C|";
    println!("{formula}");
    print_truth_table(formula);

    println!();
    let formula = "AB>";
    println!("{formula}");
    print_truth_table(formula);

    println!();
    print_truth_table("1a&");
}

#[cfg(test)]
mod tests {
    use super::formula::Formula;
    use super::*;

    #[test]
    fn subject_example() {
        let expected = "\
| A | B | C | = |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 0 |
| 1 | 0 | 1 | 1 |
| 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 |
";
        assert_eq!(truth_table("AB&C|").unwrap(), expected);
    }

    #[test]
    fn single_variable() {
        let expected = "\
| A | = |
|---|---|
| 0 | 0 |
| 1 | 1 |
";
        assert_eq!(truth_table("A").unwrap(), expected);
    }

    #[test]
    fn negation_flips_the_column() {
        let expected = "\
| A | = |
|---|---|
| 0 | 1 |
| 1 | 0 |
";
        assert_eq!(truth_table("A!").unwrap(), expected);
    }

    #[test]
    fn material_condition_is_not_commutative() {
        let expected = "\
| A | B | = |
|---|---|---|
| 0 | 0 | 1 |
| 0 | 1 | 1 |
| 1 | 0 | 0 |
| 1 | 1 | 1 |
";
        assert_eq!(truth_table("AB>").unwrap(), expected);
    }

    #[test]
    fn a_formula_without_variables_has_a_single_row() {
        let expected = "\
| = |
|---|
| 1 |
";
        assert_eq!(truth_table("10|").unwrap(), expected);
    }

    #[test]
    fn invalid_formulas_report_the_parse_error() {
        assert_eq!(truth_table(""), Err(ParseError::Empty));
        assert_eq!(truth_table("1a&"), Err(ParseError::UnknownSymbol('a')));
        assert_eq!(truth_table("AB"), Err(ParseError::MissingOperator(2)));
    }

    // The tests below target the two functions of table.rs directly.

    #[test]
    fn variables_are_sorted_and_distinct() {
        assert_eq!(parse("A").unwrap().variables(), vec!['A']);
        assert_eq!(parse("AB&").unwrap().variables(), vec!['A', 'B']);
        // B comes first in the formula, and a variable may repeat.
        assert_eq!(parse("BA&A|").unwrap().variables(), vec!['A', 'B']);
        assert_eq!(parse("ZA&").unwrap().variables(), vec!['A', 'Z']);
        assert!(parse("10&").unwrap().variables().is_empty());
    }

    #[test]
    fn rows_count_up_with_the_first_variable_as_high_bit() {
        let tree = parse("AB&").unwrap();
        let mut seen = Vec::new();
        tree.for_each_row(|values, result| seen.push((values.to_vec(), result)));

        assert_eq!(
            seen,
            vec![
                (vec![false, false], false),
                (vec![false, true], false),
                (vec![true, false], false),
                (vec![true, true], true),
            ]
        );
    }

    #[test]
    fn every_variable_of_the_alphabet_is_reachable() {
        // Z lands on the last slot of the values array.
        let tree = parse("Z").unwrap();
        let mut results = Vec::new();
        tree.for_each_row(|_, result| results.push(result));
        assert_eq!(results, vec![false, true]);
    }

    #[test]
    fn eval_reads_the_values_by_letter() {
        let mut values = [false; 26];
        values['C' as usize - 'A' as usize] = true;

        assert!(!Formula::Var('A').eval(&values));
        assert!(Formula::Var('C').eval(&values));
        assert!(parse("AC|").unwrap().eval(&values));
        assert!(!parse("AC&").unwrap().eval(&values));
    }
}
