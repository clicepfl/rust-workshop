#![allow(clippy::all)]

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn sub(a: u64, b: u64) -> u64 {
    if b > a {
        // Prevent underflow
        0
    } else {
        a - b
    }
}

fn mul_1(a: u64, mut b: u64) -> u64 {
    let mut c = 0;

    while b > 0 {
        c += a;
        b -= 1;
    }

    c
}

fn mul_2(a: u64, b: u64) -> u64 {
    let mut c = 0;

    for _ in 0..b {
        c += a
    }

    c
}

fn div(mut a: u64, b: u64) -> u64 {
    let mut r = 0;
    while a > b {
        a -= b;
        r += 1;
    }
    r
}

fn main() {
    println!("5 + 2 = {}", add(5, 2));
    println!("2 - 5 = {}", sub(2, 5));
    println!("5 * 2 = {}", mul_1(5, 2));
    println!("5 * 2 = {}", mul_2(5, 2));
    println!("5 / 2 = {}", div(5, 2));
}
