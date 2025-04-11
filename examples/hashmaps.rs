#![allow(dead_code, unused_variables, clippy::all)]

use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let blue_team = String::from("Blue");
    let blue_score = 10;

    scores.insert(blue_team, blue_score); // blue_team is moved here
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // Iterate over the HashMap
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Override a value
    scores.insert(String::from("Blue"), 25);

    // Add if not present
    scores.entry(String::from("Blue")).or_insert(50);

    // Update based on previous value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
}
