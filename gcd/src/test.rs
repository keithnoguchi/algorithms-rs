// Euclid's algorithm
//
// It's based on the fact that, given two positive integers x and y such that
// x > y, the common divisors of x and y are the same as the common divisors
// of x - y and y.
fn gcd(mut x: usize, mut y: usize) -> usize {
    while x != y {
        if x == 0 {
            return y;
        } else if y == 0 {
            return x;
        } else if x > y {
            x -= y;
        } else {
            y -= x;
        }
    }
    x
}

#[test]
fn test_gcd() {
    for x in 0..1_000 {
        for y in 0..1_000 {
            assert_eq!(super::gcd(x, y), gcd(x, y));
        }
    }
}
