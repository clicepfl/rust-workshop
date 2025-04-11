#![allow(dead_code, unused_variables, clippy::all)]

fn main() {
    // String slice : &str
    // String       : String

    let mut s = String::new();
    let data = "initial contents";

    s.push_str(data); // push a string slice
    s.push(' '); // push a char

    let s1 = String::from("hello");
    let s2 = ", world!".to_string();

    let s3 = s1 + &s2; // s1 is moved here and can no longer be used

    // Note: &String can be coerced to &str

    // Concat without moving
    let s4 = format!("{}{}", s3, s2); // s3 and s2 are not moved
    let s5 = s3.clone() + &s2; // s3 is cloned and moved

    print!("{} {} {} {}", s2, s3, s4, s5);

    // We cannot index into a String. Why ?
    let hello = String::from("hello");
    let h = &hello[0];

    // The first byte of the string is not a character
    let hello = "Здравствуйте";
    let answer = &hello[0]; // "З" is actually 2 bytes

    // Slicing Strings
    let hello = String::from("hello");
    let h = &hello[0..1]; // With our previous example, this would panic at runtime

    // Iterating over strings

    for c in "Зд".chars() {
        println!("{}", c);
    }
    // З
    // д

    for b in "Зд".bytes() {
        println!("{}", b);
    }
    // 208
    // 151
    // 208
    // 180
}
