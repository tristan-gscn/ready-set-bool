pub fn adder(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let carry = a & b;
        a ^= b;
        b = carry << 1;
    }
    a
}

fn main() {
    println!("{} + {} = {}", 3, 5, adder(3, 5));
    println!("{} + {} = {}", 4, 1, adder(4, 1));
    println!("{} + {} = {}", 0, 0, adder(0, 0));
    println!("{} + {} = {}", u32::MAX, 1, adder(u32::MAX, 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(adder(0, 0), 0);
    }

    #[test]
    fn identity() {
        assert_eq!(adder(42, 0), 42);
        assert_eq!(adder(0, 42), 42);
    }

    #[test]
    fn no_carry() {
        assert_eq!(adder(1, 2), 3);
    }

    #[test]
    fn simple_carry() {
        assert_eq!(adder(3, 5), 8);
        assert_eq!(adder(4, 1), 5);
    }

    #[test]
    fn same_operand() {
        assert_eq!(adder(7, 7), 14);
    }

    #[test]
    fn overflow_wraps() {
        assert_eq!(adder(u32::MAX, 1), 0);
        assert_eq!(adder(u32::MAX, u32::MAX), u32::MAX.wrapping_add(u32::MAX));
    }

    #[test]
    fn matches_native_addition_on_random_pairs() {
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
            assert_eq!(adder(a, b), a.wrapping_add(b));
        }
    }
}
