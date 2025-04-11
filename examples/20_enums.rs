#![allow(dead_code, unused_variables, clippy::all)]

enum WebEvent {
    PageLoad, // unit-like
    PageUnload,
    KeyPress(char), // like tuple structs
    Paste(String),
    Click { x: i64, y: i64 }, // with named fields
}

impl WebEvent {
    fn is_space_key(&self) -> bool {
        match self {
            WebEvent::KeyPress(' ') => true,
            _ => false,
        }
    }
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Binding values
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
        WebEvent::PageLoad => println!("page loaded"),
        // Exhaustiveness
        // _ => (),
    }
}

// Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_string());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    match pasted {
        WebEvent::KeyPress(c) => {
            println!("pressed '{}'.", c);
        }
        _ => ()
    }

    // if let
    if let WebEvent::KeyPress(c) = pressed {
        println!("pressed '{}'.", c);
    }

    // if let can have an else
    if let WebEvent::KeyPress(c) = pressed {
        println!("pressed '{}'.", c);
    } else {
        println!("not a key press");
    }

    // let else
    let WebEvent::KeyPress(pressed_key) = pressed else {
        // return / panic ...
        return;
    };

    println!("pressed '{}'.", pressed_key); // pressed_key is in scope here


}
