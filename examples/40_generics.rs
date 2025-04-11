#![allow(dead_code)]

use std::{fmt::Display, ops::Add};

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn print<P: Display>(value: &P) {
    println!("{}", value);
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let point_i32 = Point { x: 5_i32, y: 10 };
    let point_f32 = Point { x: 5_f32, y: 40.0 };

    let xi = point_i32.x();
    let xf = point_f32.x();

    println!("{xi}, {xf}");

    dbg!(Point { x: 1, y: 1 } + Point { x: 2, y: 2 });

    print(&"yipee");
}
