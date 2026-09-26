/// Every subset of `set`, in any order.
///
/// A set of `n` elements has `2^n` subsets — hence the O(2^n) space the
/// subject allows. Both ends count: the empty set and `set` itself are
/// subsets, so the result is never empty.
///
/// The input is assumed valid, that is free of duplicates.
pub fn powerset(set: &[i32]) -> Vec<Vec<i32>> {
    // Start from {∅} and double on each element: every subset seen so far
    // gives a second one that holds the new element too. Collecting the
    // extensions first is what lets the accumulator grow while being read.
    set.iter().fold(vec![vec![]], |mut subsets, &element| {
        let extensions: Vec<Vec<i32>> = subsets
            .iter()
            .map(|subset| {
                let mut extended = subset.clone();
                extended.push(element);
                extended
            })
            .collect();

        subsets.extend(extensions);
        subsets
    })
}

/// `{1, 2}`, or `{}` for the empty set.
fn set_notation(set: &[i32]) -> String {
    let elements: Vec<String> = set.iter().map(i32::to_string).collect();
    format!("{{{}}}", elements.join(", "))
}

fn main() {
    for set in [vec![], vec![1], vec![1, 2], vec![1, 2, 3]] {
        let subsets = powerset(&set);
        println!("P({}) holds {} subsets:", set_notation(&set), subsets.len());
        for subset in &subsets {
            println!("  {}", set_notation(subset));
        }
        println!();
    }

    // Past that size, only the count stays readable — and it is the point of
    // the O(2^n) bound.
    for n in [4, 8, 12, 16] {
        let set: Vec<i32> = (1..=n).collect();
        println!("a {n}-element set has {} subsets", powerset(&set).len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Sorts the subsets and their elements, so tests compare contents
    /// without pinning an order the subject never imposes.
    fn normalized(mut subsets: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for subset in &mut subsets {
            subset.sort_unstable();
        }
        subsets.sort_unstable();
        subsets
    }

    fn sorted(subsets: &[&[i32]]) -> Vec<Vec<i32>> {
        normalized(subsets.iter().map(|s| s.to_vec()).collect())
    }

    #[test]
    fn the_empty_set_has_exactly_one_subset() {
        // P(∅) = {∅}: one subset, not zero.
        assert_eq!(powerset(&[]), vec![Vec::<i32>::new()]);
    }

    #[test]
    fn a_singleton() {
        assert_eq!(normalized(powerset(&[1])), sorted(&[&[], &[1]]));
    }

    #[test]
    fn a_pair() {
        assert_eq!(
            normalized(powerset(&[1, 2])),
            sorted(&[&[], &[1], &[2], &[1, 2]])
        );
    }

    #[test]
    fn a_triple() {
        assert_eq!(
            normalized(powerset(&[1, 2, 3])),
            sorted(&[&[], &[1], &[2], &[3], &[1, 2], &[1, 3], &[2, 3], &[1, 2, 3],])
        );
    }

    #[test]
    fn cardinality_is_two_to_the_n() {
        for n in 0..=12u32 {
            let set: Vec<i32> = (0..n as i32).collect();
            assert_eq!(powerset(&set).len(), 2usize.pow(n), "for a {n}-element set");
        }
    }

    #[test]
    fn the_empty_set_and_the_whole_set_are_both_there() {
        let set = vec![4, 8, 15, 16];
        let subsets = normalized(powerset(&set));

        assert!(subsets.contains(&Vec::new()), "the empty set is missing");
        assert!(subsets.contains(&set), "the set itself is missing");
    }

    #[test]
    fn every_singleton_is_there() {
        let set = vec![4, 8, 15, 16, 23, 42];
        let subsets = normalized(powerset(&set));

        for element in &set {
            assert!(
                subsets.contains(&vec![*element]),
                "the singleton {{{element}}} is missing"
            );
        }
    }

    #[test]
    fn the_result_is_exactly_the_powerset() {
        // Three checks that add up to a proof: there are exactly 2^n valid
        // subsets of an n-element set, so producing 2^n of them, all
        // distinct and all valid, means producing every one of them.
        for set in [
            vec![],
            vec![7],
            vec![1, 2],
            vec![1, 2, 3, 4],
            vec![-3, 0, 12, 7, -1],
        ] {
            let subsets = powerset(&set);
            let elements: HashSet<i32> = set.iter().copied().collect();

            assert_eq!(
                subsets.len(),
                2usize.pow(set.len() as u32),
                "wrong count for {set:?}"
            );

            let distinct: HashSet<Vec<i32>> = normalized(subsets.clone()).into_iter().collect();
            assert_eq!(
                distinct.len(),
                subsets.len(),
                "duplicate subset for {set:?}"
            );

            for subset in &subsets {
                let seen: HashSet<i32> = subset.iter().copied().collect();
                assert_eq!(seen.len(), subset.len(), "{subset:?} repeats an element");
                assert!(
                    seen.is_subset(&elements),
                    "{subset:?} holds something that is not in {set:?}"
                );
            }
        }
    }

    #[test]
    fn a_subset_and_its_complement_are_both_there() {
        let set = vec![1, 2, 3, 4];
        let subsets = normalized(powerset(&set));

        for subset in &subsets {
            let complement: Vec<i32> = set
                .iter()
                .copied()
                .filter(|element| !subset.contains(element))
                .collect();

            assert!(
                subsets.contains(&complement),
                "{subset:?} is there but its complement {complement:?} is not"
            );
        }
    }

    #[test]
    fn negative_and_scattered_values_are_handled() {
        let set = vec![-42, 0, 7];
        assert_eq!(
            normalized(powerset(&set)),
            sorted(&[
                &[],
                &[-42],
                &[0],
                &[7],
                &[-42, 0],
                &[-42, 7],
                &[0, 7],
                &[-42, 0, 7],
            ])
        );
    }

    #[test]
    fn the_input_order_does_not_change_the_contents() {
        let ascending = normalized(powerset(&[1, 2, 3]));
        let descending = normalized(powerset(&[3, 2, 1]));

        assert_eq!(ascending, descending);
    }

    #[test]
    fn a_larger_set_stays_correct() {
        let set: Vec<i32> = (1..=10).collect();
        let subsets = powerset(&set);

        assert_eq!(subsets.len(), 1024);

        // Each element belongs to exactly half of the subsets.
        for element in &set {
            let holding = subsets.iter().filter(|s| s.contains(element)).count();
            assert_eq!(holding, 512, "for element {element}");
        }
    }
}
