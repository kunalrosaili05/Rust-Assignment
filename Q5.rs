5. Given a sorted array of integers, implement a function that returns the median of the array.

-->

fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid_index = len / 2;
        let median = (arr[mid_index - 1] + arr[mid_index]) as f64 / 2.0;
        median
    } else {
        
        let mid_index = len / 2;
        let median = arr[mid_index] as f64;
        median
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let median = find_median(&sorted_array);
    println!("The median of the array is {}.", median);
}

