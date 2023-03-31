//! A fibonacci number

fn fibonacci(n: usize) -> usize {
    let mut x = 1;
    let mut y = 1;
    for _ in 1..n {
        let tmp = y;
        y += x;
        x = tmp;
    }
    y
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
