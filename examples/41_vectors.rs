#![allow(dead_code, unused_variables, clippy::all)]

fn main() {
    // Creating a vector
    let mut v = vec![1, 2, 3];

    v.push(4); // v must be mutable

    let third: &i32 = &v[2];
    let hundredth: &i32 = &v[100]; // This will panic at runtime

    let third: Option<&i32> = v.get(2); // Some(3)
    let third: Option<&i32> = v.get(100); // None

    for i in &v {
      println!("{i}");
  }

    for i in &mut v {
        *i += 50;
    }

    println!("v: {:?}", v);
}

fn doesnt_compile() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0]; // immutable borrow

    v.push(6); // mutable borrow

    println!("The first element is: {first}");
}


