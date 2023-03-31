//! A power

pub fn power(base: usize, exponent: usize) -> usize {
    let mut x = 1;
    for _ in 0..exponent {
        x *= base;
    }
    x
}

#[cfg(test)]
mod test;

fn main() {
    for base in 0..=10 {
        for exponent in 0..=10 {
            println!("power({base}, {exponent}) = {}", power(base, exponent));
        }
    }
}
