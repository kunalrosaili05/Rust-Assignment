2. Given a sorted array of integers, implement a function that returns the index of the first occurence of a given number.

-->

fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    for (index, &num) in arr.iter().enumerate() {
        if num == target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 5, 6, 7, 8, 9];
    let target_number = 5;
    
    match find_first_occurrence(&sorted_array, target_number) {
        Some(index) => println!("The first occurrence of {} is at index {}.", target_number, index),
        None => println!("{} is not present in the array.", target_number),
    }
}
