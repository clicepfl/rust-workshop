#![allow(dead_code, unused_variables, unreachable_code, clippy::all)]

fn main() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    // Types for the closures are inferred from the context
    add_one_v3(1);
    add_one_v4(1);
}

fn borrows() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");
}

fn mutably_borrows() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    // Here we are borrowing the list mutably

    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");
}

use std::thread;

fn takes_ownership() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // This closure takes ownership of the list using move
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
}


fn fn_traits() {
  // Function that takes a FnOnce closure as an argument
  let result = Some(2).unwrap_or_else(|| {
    println!("Value is None");
    0
  });

  // Function that takes a FnMut closure as an argument
  let mut count = 0;
  let result = [1, 2, 3].map(|_| {
    count += 1;
    println!("Value is Some");
    count
  });


}
