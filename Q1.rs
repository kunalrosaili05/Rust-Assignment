1. Implement a function that checks whether a given string is a palindrome or not.

--->

fn is_palindrome(input: &str) -> bool {
    input.chars().eq(input.chars().rev()) 
}

fn main() {
    let test_str = "racecar";
    if is_palindrome(test_str) {
        println!("{} is a palindrome.", test_str);
    } else {
        println!("{} is not a palindrome.", test_str);
    }
}

