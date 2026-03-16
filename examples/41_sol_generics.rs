#![allow(dead_code, unused_imports)]

use std::{fmt::Display, ops::Add, process::Output};

/// 1. Modify this code so that we can create a Point where x and y have
///    different types. Make sure to update print and add too !
/// 2. Add a mixup function to Point, that can be called with another Point
///    as argument and returns a new Point where x is from the first and y
///    is from the second.

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: other.y }
    }
}

fn print<P: Display>(value: &P) {
    println!("{}", value);
}

impl<T: Add<Output = T>, U: Add<Output = U>> Add for Point<T, U> {
    type Output = Point<T, U>;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    // 1. Uncomment for part 1
    let point_a = Point { x: 5.5, y: 10 };
    let point_b = Point { x: "Hello", y: 'c' };

    // 2. Uncomment for part 2
    let mixup = point_a.mixup(point_b);
    println!("{:?}", mixup);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_different_types() {
        // Support different types for x and y
        let p = Point { x: 5.5, y: 6 };
        assert_eq!(p.x, 5.5);
        assert_eq!(p.y, 6);
    }

    #[test]
    fn test_mixup_integers_and_strings() {
        // Mixing a Point<f64, i32> with a Point<&str, char>
        let p1 = Point { x: 5.5, y: 6 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        // Should take x from p1 (f64) and y from p2 (char)
        assert_eq!(p3.x, 5.5);
        assert_eq!(p3.y, 'c');
    }

    #[test]
    fn test_mixup_same_types() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = Point { x: 3, y: 4 };
        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 1);
        assert_eq!(p3.y, 4);
    }
}
