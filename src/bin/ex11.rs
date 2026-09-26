// The inverse needs the scale its counterpart used, and the tests need map
// itself to check the round trip. Same #[path] reuse as ex01 does on ex00.
#[allow(dead_code)]
#[path = "ex10.rs"]
mod ex10;

use ex10::{map, SCALE};

/// Recovers the pair of coordinates a value in `[0, 1)` was built from.
///
/// Scaling back up rebuilds the packed integer exactly — the scale is a power
/// of two, so neither direction rounds — and the two coordinates are read
/// back out of their halves.
///
/// Out of range, the subject leaves the behaviour undefined; this reports the
/// value and falls back on `(0, 0)`. Note that `1.0` is out of range: `map`
/// never produces it, since its image is `[0, 1)`.
pub fn reverse_map(n: f64) -> (u16, u16) {
    if !(0.0..1.0).contains(&n) {
        eprintln!("error: {n} is outside [0, 1)");
        return (0, 0);
    }

    let packed = (n * SCALE) as u64;

    ((packed >> 16) as u16, packed as u16)
}

fn main() {
    println!("{:>7} {:>7}   {:>26}   reverse_map", "x", "y", "map(x, y)");
    for (x, y) in [
        (0, 0),
        (0, 1),
        (1, 0),
        (42, 4242),
        (32768, 32768),
        (65535, 65535),
    ] {
        let n = map(x, y);
        println!("{x:>7} {y:>7}   {n:>26}   {:?}", reverse_map(n));
    }

    println!();
    // The two round trips the subject asks for.
    let (x, y) = (4242, 2424);
    println!(
        "reverse_map(map({x}, {y})) == ({x}, {y}) : {}",
        reverse_map(map(x, y)) == (x, y)
    );

    let n = 12345.0 / SCALE;
    println!("map(reverse_map({n})) == {n} : {}", {
        let (x, y) = reverse_map(n);
        map(x, y) == n
    });

    println!();
    // Everything the subject leaves undefined, reported rather than guessed.
    for n in [-0.5, 1.0, 1.5, f64::NAN, f64::INFINITY] {
        let _ = reverse_map(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn xorshift() -> impl FnMut() -> u64 {
        let mut rng: u64 = 0x9E3779B97F4A7C15;
        move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        }
    }

    #[test]
    fn known_values() {
        assert_eq!(reverse_map(0.0), (0, 0));
        assert_eq!(reverse_map(1.0 / SCALE), (0, 1));
        assert_eq!(reverse_map(65536.0 / SCALE), (1, 0));
        assert_eq!(reverse_map((SCALE - 1.0) / SCALE), (65535, 65535));
    }

    #[test]
    fn round_trip_on_the_corners() {
        for (x, y) in [(0, 0), (0, 65535), (65535, 0), (65535, 65535)] {
            assert_eq!(reverse_map(map(x, y)), (x, y));
        }
    }

    #[test]
    fn round_trip_exhaustively_on_a_corner_of_the_grid() {
        // 256 x 256 = 65536 pairs, every one of them recovered.
        for x in 0..256u16 {
            for y in 0..256u16 {
                assert_eq!(reverse_map(map(x, y)), (x, y), "for ({x}, {y})");
            }
        }
    }

    #[test]
    fn round_trip_over_the_whole_range() {
        let mut next = xorshift();

        for _ in 0..100_000 {
            let (x, y) = (next() as u16, next() as u16);
            assert_eq!(reverse_map(map(x, y)), (x, y), "for ({x}, {y})");
        }
    }

    #[test]
    fn round_trip_the_other_way_round() {
        // (f o f^-1)(n) == n, for the values that are actually in the image.
        let mut next = xorshift();

        for _ in 0..100_000 {
            let n = (next() as u32) as f64 / SCALE;
            let (x, y) = reverse_map(n);

            assert_eq!(map(x, y), n, "for {n}");
        }
    }

    #[test]
    fn both_halves_are_read_from_the_right_bits() {
        // x rides the high 16 bits, y the low ones.
        assert_eq!(reverse_map(map(0xABCD, 0x1234)), (0xABCD, 0x1234));
        assert_eq!(reverse_map(0xABCD1234u32 as f64 / SCALE), (0xABCD, 0x1234));
    }

    #[test]
    fn out_of_range_inputs_fall_back_without_panicking() {
        for n in [
            -1.0,
            -0.5,
            -f64::EPSILON,
            1.0,
            1.5,
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::MAX,
        ] {
            assert_eq!(reverse_map(n), (0, 0), "for {n}");
        }
    }

    #[test]
    fn one_is_out_of_range_since_map_never_produces_it() {
        assert!(map(65535, 65535) < 1.0);
        assert_eq!(reverse_map(1.0), (0, 0));
    }

    #[test]
    fn a_value_between_two_cells_falls_back_on_the_lower_one() {
        // Only values of the form k / 2^32 are in the image; anything in
        // between is truncated down to the cell it sits in.
        let between = 3.5 / SCALE;

        assert_eq!(reverse_map(between), (0, 3));
        assert_eq!(reverse_map(3.0 / SCALE), (0, 3));
    }
}
