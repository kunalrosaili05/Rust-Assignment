3. Given a string of words, implement a function that returns the shortest word in the string.

-->

fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    let input_str = "The quick brown fox jumps over the lazy dog";
    
    match shortest_word(input_str) {
        Some(shortest) => println!("The shortest word is '{}'.", shortest),
        None => println!("No words found in the input string."),
    }
}
