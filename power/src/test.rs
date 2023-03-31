fn power(base: usize, exponent: usize) -> usize {
    if exponent == 0 {
        return 1;
    }
    power(base, exponent - 1) * base
}

#[test]
fn test_power() {
    for base in 0..=10 {
        for exponent in 0..=10 {
            assert_eq!(super::power(base, exponent), power(base, exponent));
        }
    }
}
