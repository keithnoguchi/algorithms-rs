//! A greatest common divisor

#![forbid(missing_debug_implementations)]

fn gcd(mut x: usize, mut y: usize) -> usize {
    // https://en.wikipedia.org/wiki/Greatest_common_divisor
    if x == 0 {
        return y;
    }
    while y != 0 {
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        y %= x;
    }
    x
}

#[cfg(test)]
mod test;

fn main() {
    let mut args = std::env::args().skip(1);
    let x = args.next().and_then(|x| x.parse().ok()).unwrap_or(10);
    let y = args.next().and_then(|y| y.parse().ok()).unwrap_or(20);

    println!("gcd({x}, {y}) = {}", gcd(x, y));
}
