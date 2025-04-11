fn skip_first_word(s: &str) -> &str {
    let ws_index = s.chars().take_while(|c| *c != ' ').count();

    &s[ws_index..]
}

fn main() {
    let sentence = String::from("Hello world!");
    let skipped = skip_first_word(&sentence);

    drop(sentence);

    println!("{skipped}");
}
