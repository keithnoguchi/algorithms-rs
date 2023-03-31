//! A fibonacci sequence
//!
//! https://en.wikipedia.org/wiki/Fibonacci_sequence

fn fibonacci(n: usize) -> usize {
    let mut x = 0;
    let mut y = 1;
    for _ in 0..n {
        let tmp = y;
        y += x;
        x = tmp;
    }
    x
}

#[cfg(test)]
mod test;

fn main() {
    let max = std::env::args()
        .nth(1)
        .and_then(|n| n.parse().ok())
        .unwrap_or(92);

    for n in 0..=max {
        println!("fibonacci({n}) = {}", fibonacci(n));
    }
}
