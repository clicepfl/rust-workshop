fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

#[test]
fn test_tuple() {
    let rect1 = (30, 50);
    let rect2 = (10, 20);
    assert_eq!(area(rect1), area(rect2));
}

#[test]
fn test_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    assert_eq!(area(&rect1), area(&rect2));
}

// Print a rectangle using Debug derived trait
#[test]
fn test_debug() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {rect1:?}");
    dbg!(&rect1);
}

// Make area a method of the Rectangle struct
#[test]
fn test_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// Write a new method can_hold that takes another Rectangle and returns true if the first can hold the second
#[test]
fn test_can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 20,
    };
    assert!(rect1.can_hold(&rect2));
}

// Write an associated function new that creates a square Rectangle
#[test]
fn test_square() {
    let square = Rectangle::square(10);
    assert_eq!(square.width, 10);
    assert_eq!(square.height, 10);
}
