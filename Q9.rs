9. Reverse a string in Rust.

-->


fn reverse_string(input: &str) -> String {
    input.chars().rev().collect::<String>()
}

fn main() {
    let original_str = "Hello, world!";
    let reversed_str = reverse_string(original_str);
    println!("Original string: {}", original_str);
    println!("Reversed string: {}", reversed_str);
}
