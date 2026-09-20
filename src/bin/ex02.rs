pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

fn main() {
    for n in 0..=8 {
        println!("gray_code({}) = {}", n, gray_code(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subject_examples() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }

    #[test]
    fn successive_values_differ_by_single_bit() {
        for i in 0..10_000 {
            let diff = gray_code(i) ^ gray_code(i + 1);
            assert_eq!(diff.count_ones(), 1);
        }
    }

    #[test]
    fn max_value() {
        assert_eq!(gray_code(u32::MAX), u32::MAX ^ (u32::MAX >> 1));
    }
}
