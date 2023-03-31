//! A factorial sequence

fn factorial(n: usize) -> usize {
    let mut x = 1;
    for i in 2..=n {
        x *= i;
    }
    x
}

#[cfg(test)]
mod test;

fn main() {
    for n in 0..=20 {
        println!("factorial({n}) = {}", factorial(n));
    }
}
