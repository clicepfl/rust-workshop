fn first_word(s: &str) -> &str {
    let mut ws_index = 0usize;
    while s.chars().nth(ws_index).unwrap() != ' ' {
        ws_index += 1;
    }
    &s[0..ws_index]
}

fn main() {
    let s = String::from("Hello world!");
    let word = first_word(&s);
    drop(s);
    println!("{word}");
}
