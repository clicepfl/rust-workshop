#![allow(dead_code, unused_variables, unreachable_code, clippy::all)]

use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn greeting() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // unwrap
    let uwu_file_result = File::open("uwu.txt");
    let uwu_file = uwu_file_result.unwrap();

    // expect - give context about why we think it will always succeed
    let owo_file_result = File::open("owo.txt");
    let owo_file = owo_file_result.expect("Problem opening the file");
}

// Propagating errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Using ?
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // Return type must be Result / OPtion / ... to use ?
}

// Chaining
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Using fs
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// Option with ?
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// You can use ? on an Option in a function that returns an Option
// Or ? on a Result in a function that returns a Result
// But not mix and match !

// To use ? on an option in a function that returns a Result
// use the ok_or method to convert an Option into a Result
fn last_char_of_first_line2(text: &str) -> Result<char, io::Error> {
    text.lines()
        .next()
        .ok_or(io::Error::new(ErrorKind::Other, "No lines"))?
        .chars()
        .last()
        .ok_or(io::Error::new(ErrorKind::Other, "No chars"))
}

// The other way around
// use the ok method to convert a Result into an Option
fn read_username_from_file5() -> Option<String> {
    let mut username = String::new();

    File::open("hello.txt")
        .ok()?
        .read_to_string(&mut username)
        .ok()?;

    Some(username)
}

// Main can return a Result
// If the result is an error, the program will exit with a non-zero status code
fn main() -> Result<(), io::Error> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
