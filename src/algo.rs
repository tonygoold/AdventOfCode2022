pub fn gcd(mut x: usize, mut y: usize) -> usize {
    let mut t;
    while y != 0 {
        t = x;
        x = y;
        y = t % y;
    }
    x
}

pub fn lcm(x: usize, y: usize) -> usize {
    let g = gcd(x, y);
    let prod = x * y;
    prod / g
}
