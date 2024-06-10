fn gcd(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut u, mut g, mut x, mut y) = (1, a, 0, b);

    while y != 0 {
        let q = g / y;
        let t = g % y;

        let s = u - q * x;
        u = x;
        g = y;
        x = s;
        y = t;
    }

    let v = (g - a * u) / b;
    (g, u, v)
}

fn main() {
    let a = 56;
    let b = 98;
    let (g, u, v) = gcd(a, b);
    println!("gcd({}, {}) = {}", a, b, g);
    println!("Solution to {}u + {}v = {} is (u, v) = ({}, {})", a, b, g, u, v);
}
