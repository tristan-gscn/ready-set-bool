fn multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    for i in 0..32 {

    }
    0
}

fn main() {
    println!("{} * {} = {}", 3, 4, multiplier(3, 4));
    println!("{} * {} = {}", 7, 6, multiplier(7, 6));
    println!("{} * {} = {}", 0, 5, multiplier(0, 5));
    println!("{} * {} = {}", u32::MAX, 2, multiplier(u32::MAX, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(5, 0), 0);
        assert_eq!(multiplier(0, 5), 0);
    }

    #[test]
    fn identity() {
        assert_eq!(multiplier(1, 42), 42);
        assert_eq!(multiplier(42, 1), 42);
    }

    #[test]
    fn simple() {
        assert_eq!(multiplier(3, 4), 12);
        assert_eq!(multiplier(7, 6), 42);
    }

    #[test]
    fn same_operand() {
        assert_eq!(multiplier(7, 7), 49);
    }

    #[test]
    fn overflow_wraps() {
        assert_eq!(multiplier(u32::MAX, 2), u32::MAX.wrapping_mul(2));
        assert_eq!(multiplier(u32::MAX, u32::MAX), u32::MAX.wrapping_mul(u32::MAX));
    }

    #[test]
    fn matches_native_multiplication_on_random_pairs() {
        let mut rng: u64 = 0x2545F4914F6CDD1D;
        let mut next_u32 = || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            (rng & 0xFFFF_FFFF) as u32
        };

        for _ in 0..10_000 {
            let a = next_u32();
            let b = next_u32();
            assert_eq!(multiplier(a, b), a.wrapping_mul(b));
        }
    }
}
