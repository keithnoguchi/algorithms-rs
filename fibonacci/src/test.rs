fn fibonacci(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    fibonacci(n - 2) + fibonacci(n - 1)
}

#[test]
fn test_fibonacci() {
    for n in 0..=40 {
        assert_eq!(super::fibonacci(n), fibonacci(n), "n={n}");
    }
}
