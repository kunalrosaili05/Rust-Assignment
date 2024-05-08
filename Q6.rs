6. Implement a function that finds the longest common prefix of a given set of strings.

-->

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new(); 
    }

    let first_string = strings[0];
    let mut longest_prefix = String::new();

    'outer: for (index, char) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(index) {
                if c != char {
                    break 'outer; 
                }
            } else {
                break 'outer; 
            }
        }
        longest_prefix.push(char);
    }

    longest_prefix
}

fn main() {
    let strings = vec!["apple", "app", "application", "aptitude"];
    let common_prefix = longest_common_prefix(&strings);
    println!("The longest common prefix is '{}'.", common_prefix);
}
