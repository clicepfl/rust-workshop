#![allow(dead_code, unused_variables, clippy::all)]

struct TricolorState {
    color: Color,
    ascending: bool,
}

enum Color {
    Red,
    Yellow,
    Green,
}

impl TricolorState {
    fn green() -> Self {
        Self {
            color: Color::Green,
            ascending: false,
        }
    }
    fn red() -> Self {
        Self {
            color: Color::Red,
            ascending: false,
        }
    }

    fn next(&self) -> Self {
        match self.color {
            Color::Red => Self {
                color: Color::Yellow,
                ascending: false,
            },
            Color::Yellow => {
                if self.ascending {
                    Self {
                        color: Color::Red,
                        ascending: false,
                    }
                } else {
                    Self {
                        color: Color::Green,
                        ascending: false,
                    }
                }
            }
            Color::Green => Self {
                color: Color::Yellow,
                ascending: true,
            },
        }
    }

    fn color(&self) -> &Color {
        &self.color
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
