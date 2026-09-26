/// 2^32: as many values as there are `(x, y)` pairs to encode.
///
/// A power of two, so dividing by it only decrements the exponent of an f64
/// and leaves its mantissa untouched. No rounding happens, which is what
/// makes the mapping provably injective rather than injective by luck.
pub const SCALE: f64 = 4_294_967_296.0;

/// Sends a pair of coordinates to a unique value in `[0, 1)`.
///
/// The coordinates are packed into disjoint halves of a 32-bit integer —
/// `x` takes the high bits, `y` the low ones — which is then scaled down.
/// Both steps are exact: 32 bits fit in the 53-bit mantissa of an f64, and
/// the divisor is a power of two. The result is therefore bijective onto its
/// image, as the subject requires.
///
/// The `u16` parameters make an out-of-range input impossible to express, so
/// there is nothing to reject here.
pub fn map(x: u16, y: u16) -> f64 {
    let packed = (x as u64) << 16 | y as u64;
    packed as f64 / SCALE
}

fn main() {
    println!("{:>7} {:>7}   {:>12}   map(x, y)", "x", "y", "packed");
    for (x, y) in [
        (0, 0),
        (0, 1),
        (0, 65535),
        (1, 0),
        (1, 1),
        (32768, 0),
        (65535, 65534),
        (65535, 65535),
    ] {
        let packed = (x as u64) << 16 | y as u64;
        println!("{x:>7} {y:>7}   {packed:>12}   {}", map(x, y));
    }

    println!();
    // Packing x above y makes the order lexicographic: y counts up first,
    // and rolling it over moves to the next x.
    println!("map(0, 65535) < map(1, 0) : {}", map(0, 65535) < map(1, 0));

    println!();
    // Nothing is rounded along the way, so the value scaled back up lands
    // exactly on the integer it came from.
    let scaled = map(65535, 65535) * SCALE;
    println!(
        "map(65535, 65535) * 2^32 = {scaled} (exact: {})",
        scaled == scaled.trunc()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn xorshift() -> impl FnMut() -> u64 {
        let mut rng: u64 = 0x2545F4914F6CDD1D;
        move || {
            rng ^= rng << 13;
            rng ^= rng >> 7;
            rng ^= rng << 17;
            rng
        }
    }

    #[test]
    fn known_values() {
        assert_eq!(map(0, 0), 0.0);
        assert_eq!(map(0, 1), 1.0 / SCALE);
        assert_eq!(map(1, 0), 65536.0 / SCALE);
        assert_eq!(map(65535, 65535), (SCALE - 1.0) / SCALE);
    }

    #[test]
    fn every_value_lands_in_the_unit_interval() {
        let mut next = xorshift();

        for _ in 0..10_000 {
            let value = map(next() as u16, next() as u16);
            assert!((0.0..1.0).contains(&value), "{value} escaped [0, 1)");
        }
    }

    #[test]
    fn the_scaling_never_rounds() {
        // Scaled back up, every value lands exactly on its packed integer.
        let mut next = xorshift();

        for _ in 0..10_000 {
            let (x, y) = (next() as u16, next() as u16);
            let packed = map(x, y) * SCALE;

            assert_eq!(packed, packed.trunc(), "for ({x}, {y})");
            assert_eq!(packed as u64, (x as u64) << 16 | y as u64);
        }
    }

    #[test]
    fn distinct_pairs_give_distinct_values() {
        // Exhaustive over a 256 x 256 corner: 65536 pairs, no collision.
        let mut seen = HashSet::with_capacity(65536);

        for x in 0..256u16 {
            for y in 0..256u16 {
                assert!(seen.insert(map(x, y).to_bits()), "({x}, {y}) collided");
            }
        }

        assert_eq!(seen.len(), 65536);
    }

    #[test]
    fn distinct_pairs_stay_distinct_across_the_whole_range() {
        let mut next = xorshift();
        let mut seen = HashSet::new();
        let mut pairs = HashSet::new();

        for _ in 0..100_000 {
            let (x, y) = (next() as u16, next() as u16);

            // Only a fresh pair is expected to yield a fresh value.
            if pairs.insert((x, y)) {
                assert!(seen.insert(map(x, y).to_bits()), "({x}, {y}) collided");
            }
        }

        assert_eq!(seen.len(), pairs.len());
    }

    #[test]
    fn the_edges_of_the_range_do_not_collide() {
        let edges = [0u16, 1, 255, 256, 32767, 32768, 65534, 65535];
        let mut seen = HashSet::new();

        for x in edges {
            for y in edges {
                assert!(seen.insert(map(x, y).to_bits()), "({x}, {y}) collided");
            }
        }

        assert_eq!(seen.len(), edges.len() * edges.len());
    }

    #[test]
    fn the_order_is_lexicographic() {
        // y counts up first; rolling it over moves on to the next x.
        assert!(map(0, 0) < map(0, 1));
        assert!(map(0, 65534) < map(0, 65535));
        assert!(map(0, 65535) < map(1, 0));
        assert!(map(1, 0) < map(1, 1));
        assert!(map(65534, 65535) < map(65535, 0));
    }

    #[test]
    fn the_image_reaches_both_ends_without_touching_one() {
        // 0 is attained, 1 is not: the image is [0, 1), a subset of [0, 1].
        assert_eq!(map(0, 0), 0.0);
        assert!(map(65535, 65535) < 1.0);
        // ...and it comes as close to 1 as the encoding allows.
        assert_eq!(1.0 - map(65535, 65535), 1.0 / SCALE);
    }
}
