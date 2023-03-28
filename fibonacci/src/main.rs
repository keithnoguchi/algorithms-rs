//! A fibonacci number

#![forbid(unsafe_code, missing_debug_implementations)]

pub fn fibonacci(n: usize) -> usize {
    let mut a = 1;
    let mut b = 1;

    for _ in 1..n {
        let tmp = b;
        b += a;
        a = tmp;
    }
    b
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
