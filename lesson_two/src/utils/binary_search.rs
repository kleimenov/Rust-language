// Binary search algorithm

/*
Binary Search is an efficient algorithm for finding an item from a sorted list of items. 
It works by repeatedly dividing the search interval in half. 
If the value of the search key is less than the item in the middle of the interval, narrow the interval to the lower half. 
Otherwise, narrow it to the upper half. Repeatedly check until the value is found or the interval is empty.

Steps:
1. Start with a sorted array.
2. Compare the target value with the middle element of the array.
 - If the target value matches the middle element, return its index.
 - If the target value is less than the middle element, repeat the search in the left half.
 - If the target value is greater than the middle element, repeat the search in the right half.
3. Repeat the process until the target value is found or the search interval is empty.

Time Complexity:
O(log n): The search space is halved with each step, making it very efficient for large datasets.

Space Complexity:
O(1): Binary search uses constant space for iterative implementations.
*/

pub fn binary_search_example_1(arr: &[u32], target: u32) -> Option<usize> {
    let mut lower_bound = 0; // Start of the array will reference the first element and consider as lower bound
    let mut upper_bound = arr.len() - 1; // End of the array will reference the last element and consider as upper bound

    while lower_bound <= upper_bound{ // loop untill the lower bound is less than or equal to upper bound
        println!("Lower bound: {}, Upper bound: {}, Magic {}", lower_bound, upper_bound, (upper_bound - lower_bound) / 2);
        let mid = lower_bound + (upper_bound - lower_bound) / 2; // Find the middle element of the array by adding lower bound and upper bound and divide by 2
        println!("Middle element: {}", mid);
        if arr[mid] == target{ // If the middle element is equal to the target value
            return Some(mid); // Return the index of the middle element
        } else if arr[mid] < target{ // If the middle element is less than the target value
            lower_bound = mid + 1; // Move the lower bound to the right of the middle element
        } else { // If the middle element is greater than the target value
            upper_bound = mid - 1; // Move the upper bound to the left of the middle element
        }
    }
    None // Return None if the target value is not found in the array
}

// pub fn binary_search_example_2() -> {

// }