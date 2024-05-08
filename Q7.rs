7. Implement a function that returns the kth smallest element in a given array.

-->


fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None; 
    }

    let mut sorted_arr = arr.to_vec(); 
    sorted_arr.sort(); 

    Some(sorted_arr[k - 1]) 
}

fn main() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k = 4;
    
    match kth_smallest_element(&array, k) {
        Some(kth_smallest) => println!("The {}th smallest element is {}.", k, kth_smallest),
        None => println!("Invalid value of k."),
    }
}