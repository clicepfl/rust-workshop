#![allow(dead_code, unused_variables, clippy::all)]

// Create a TricolorState struct
// Create a Color enum

impl TricolorState {
    fn green() -> Self {
        todo!()
    }
    fn red() -> Self {
        todo!()
    }

    fn next(&self) -> Self {
        todo!()
    }

    fn color(&self) -> &Color {
        todo!()
    }
}

#[test]
fn green_color() {
    assert!(matches!(TricolorState::green().color(), Color::Green));
}

#[test]
fn red_color() {
    assert!(matches!(TricolorState::red().color(), Color::Red));
}

#[test]
fn red_to_green() {
    let red = TricolorState::red();
    assert!(matches!(red.color(), Color::Red));

    let yellow = red.next();
    assert!(matches!(yellow.color(), Color::Yellow));

    let green = yellow.next();
    assert!(matches!(green.color(), Color::Green));
}

#[test]
fn green_to_red() {
    let green = TricolorState::green();
    assert!(matches!(green.color(), Color::Green));

    let yellow = green.next();
    assert!(matches!(yellow.color(), Color::Yellow));

    let red = yellow.next();
    assert!(matches!(red.color(), Color::Red));
}

fn main() {}
