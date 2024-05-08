10. Check if a number is prime in Rust.

-->


fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false; 
    }

    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false; 
        }
    }

    true 
}

fn main() {
    let test_number = 17;
    if is_prime(test_number) {
        println!("{} is a prime number.", test_number);
    } else {
        println!("{} is not a prime number.", test_number);
    }
}

