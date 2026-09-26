// Shared module: each exercise uses a subset of it.
#[allow(dead_code, unused_imports)]
#[path = "../formula/mod.rs"]
mod formula;

use formula::parse;

pub fn eval_set(formula: &str, sets: &[Vec<i32>]) -> Vec<i32> {
    parse(formula)
        .map_err(|reason| reason.to_string())
        .and_then(|tree| tree.eval_set(sets).map_err(|reason| reason.to_string()))
        .unwrap_or_else(|reason| {
            eprintln!("error: {formula:?}: {reason}");
            Vec::new()
        })
}

/// `{1, 2}`, or `{}` for the empty set.
fn set_notation(set: &[i32]) -> String {
    let elements: Vec<String> = set.iter().map(i32::to_string).collect();
    format!("{{{}}}", elements.join(", "))
}

fn show(formula: &str, sets: &[Vec<i32>]) {
    let given: Vec<String> = sets.iter().map(|set| set_notation(set)).collect();
    println!("{formula:<6} over {}", given.join(", "));
    println!("     = {}", set_notation(&eval_set(formula, sets)));
}

fn main() {
    // The three examples from the subject.
    show("AB&", &[vec![0, 1, 2], vec![0, 3, 4]]);
    show("AB|", &[vec![0, 1, 2], vec![3, 4, 5]]);
    show("A!", &[vec![0, 1, 2]]);

    println!();
    // A complement is only ever taken within the union of what is given:
    // here that union is {0,1,2,3,4}, so !A comes out as {3, 4}.
    show("A!B&", &[vec![0, 1, 2], vec![3, 4]]);
    show("AB^", &[vec![0, 1, 2], vec![1, 2, 3]]);
    show("AB>", &[vec![0, 1], vec![1, 2]]);

    println!();
    // Both cases the subject leaves undefined, reported rather than guessed.
    let _ = eval_set("AB&", &[vec![0]]);
    let _ = eval_set("AB", &[vec![0], vec![1]]);
}

#[cfg(test)]
mod tests {
    use super::formula::{Op, SetEvalError};
    use super::*;

    /// The subject states the order of the result does not matter.
    fn sorted(mut set: Vec<i32>) -> Vec<i32> {
        set.sort_unstable();
        set
    }

    fn eval_sorted(formula: &str, sets: &[Vec<i32>]) -> Vec<i32> {
        sorted(eval_set(formula, sets))
    }

    #[test]
    fn subject_examples() {
        assert_eq!(eval_set("AB&", &[vec![0, 1, 2], vec![0, 3, 4]]), vec![0]);
        assert_eq!(
            eval_sorted("AB|", &[vec![0, 1, 2], vec![3, 4, 5]]),
            vec![0, 1, 2, 3, 4, 5]
        );
        assert_eq!(eval_set("A!", &[vec![0, 1, 2]]), Vec::<i32>::new());
    }

    #[test]
    fn a_lone_variable_gives_its_set_back() {
        assert_eq!(eval_sorted("A", &[vec![2, 0, 1]]), vec![0, 1, 2]);
    }

    #[test]
    fn conjunction_is_the_intersection() {
        assert_eq!(
            eval_sorted("AB&", &[vec![0, 1, 2, 3], vec![2, 3, 4]]),
            vec![2, 3]
        );
        assert_eq!(
            eval_set("AB&", &[vec![0, 1], vec![2, 3]]),
            Vec::<i32>::new()
        );
    }

    #[test]
    fn disjunction_is_the_union() {
        assert_eq!(eval_sorted("AB|", &[vec![0, 1], vec![1, 2]]), vec![0, 1, 2]);
    }

    #[test]
    fn negation_is_the_complement_within_the_union() {
        // With a single set the encompassing set is that set, so !A is empty
        // — the subject's own third example.
        assert_eq!(eval_set("A!", &[vec![0, 1, 2]]), Vec::<i32>::new());

        // Bring a second set and the encompassing set becomes {0,1,2,3,4},
        // so !A is {3,4} rather than every other integer. B is intersected
        // in only to keep as many variables as sets, as the subject asks.
        assert_eq!(
            eval_sorted("A!B&", &[vec![0, 1, 2], vec![3, 4]]),
            vec![3, 4]
        );
        // !(A & B) leaves everything but the intersection.
        assert_eq!(eval_sorted("AB&!", &[vec![0, 1], vec![1, 2]]), vec![0, 2]);
    }

    #[test]
    fn exclusive_disjunction_is_the_symmetric_difference() {
        assert_eq!(
            eval_sorted("AB^", &[vec![0, 1, 2], vec![1, 2, 3]]),
            vec![0, 3]
        );
    }

    #[test]
    fn material_condition_and_equivalence() {
        // A > B is !A | B: everything outside A, plus B.
        assert_eq!(eval_sorted("AB>", &[vec![0, 1], vec![1, 2]]), vec![1, 2]);
        // A = B is where both agree, inside the encompassing set.
        assert_eq!(eval_sorted("AB=", &[vec![0, 1], vec![1, 2]]), vec![1]);
    }

    #[test]
    fn three_sets() {
        let sets = [vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]];
        // (A & B) | C
        assert_eq!(eval_sorted("AB&C|", &sets), vec![1, 2, 3, 4]);
        // A & B & C
        assert_eq!(eval_sorted("AB&C&", &sets), vec![2]);
    }

    #[test]
    fn duplicates_in_the_input_do_not_reach_the_result() {
        assert_eq!(eval_sorted("AB|", &[vec![0, 0, 1], vec![1, 1]]), vec![0, 1]);
    }

    #[test]
    fn set_identities_hold() {
        let sets = [vec![0, 1, 2, 3], vec![2, 3, 4, 5]];
        let a = sorted(sets[0].clone());
        let universe = vec![0, 1, 2, 3, 4, 5];

        // A & A == A and A | A == A, over a single set.
        assert_eq!(eval_sorted("AA&", &[sets[0].clone()]), a);
        assert_eq!(eval_sorted("AA|", &[sets[0].clone()]), a);

        // A | !A is everything, A & !A is nothing.
        assert_eq!(eval_sorted("AA!|", &sets[..1]), a);
        assert_eq!(eval_set("AA!&", &sets[..1]), Vec::<i32>::new());

        // De Morgan, read on sets this time.
        assert_eq!(eval_sorted("AB|!", &sets), eval_sorted("A!B!&", &sets));
        assert_eq!(eval_sorted("AB&!", &sets), eval_sorted("A!B!|", &sets));

        // The intersection is inside both, the union holds both.
        let intersection = eval_sorted("AB&", &sets);
        let union = eval_sorted("AB|", &sets);
        assert!(intersection.iter().all(|e| sets[0].contains(e)));
        assert!(intersection.iter().all(|e| sets[1].contains(e)));
        assert_eq!(union, universe);
    }

    #[test]
    fn normal_forms_evaluate_to_the_same_set() {
        let sets = [vec![0, 1, 2, 3], vec![2, 3, 4], vec![1, 3, 5]];

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

            // Only formulas naming exactly A, B and C can be evaluated here.
            if tree.variables() != vec!['A', 'B', 'C'] {
                continue;
            }

            let expected = tree.eval_set(&sets).unwrap();
            assert_eq!(
                tree.nnf().eval_set(&sets),
                Ok(expected.clone()),
                "{formula}"
            );
            assert_eq!(tree.cnf().eval_set(&sets), Ok(expected), "{formula}");
        }
    }

    #[test]
    fn the_set_count_must_match_the_variable_count() {
        let tree = parse("AB&").unwrap();

        assert_eq!(
            tree.eval_set(&[vec![0]]),
            Err(SetEvalError::CountMismatch {
                variables: 2,
                sets: 1
            })
        );
        assert_eq!(
            tree.eval_set(&[vec![0], vec![1], vec![2]]),
            Err(SetEvalError::CountMismatch {
                variables: 2,
                sets: 3
            })
        );
    }

    #[test]
    fn a_gap_in_the_letters_is_rejected() {
        // A and C name two variables, but C indexes a third set.
        let tree = parse("AC&").unwrap();
        assert_eq!(
            tree.eval_set(&[vec![0], vec![1]]),
            Err(SetEvalError::NoSetFor('C'))
        );
    }

    #[test]
    fn invalid_input_yields_an_empty_set() {
        assert_eq!(eval_set("", &[]), Vec::<i32>::new());
        assert_eq!(eval_set("AB", &[vec![0], vec![1]]), Vec::<i32>::new());
        assert_eq!(eval_set("Aa&", &[vec![0]]), Vec::<i32>::new());
        assert_eq!(eval_set("AB&", &[vec![0]]), Vec::<i32>::new());
    }

    /// A random well-formed formula over A..C, in reverse polish notation.
    fn random_formula(next: &mut impl FnMut() -> u64, depth: u32) -> String {
        if depth == 0 || next().is_multiple_of(4) {
            return ((b'A' + (next() % 3) as u8) as char).to_string();
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
