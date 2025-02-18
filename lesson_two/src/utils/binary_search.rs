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
        let mid = lower_bound + (upper_bound - lower_bound) / 2; // Find the middle element of the array by adding lower bound and upper bound and divide by 2
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

pub fn binary_search_example_2(arr: &[u32 ], desired_values: u32) -> Option<(u32, usize)>{
    let mut lower_bounder: usize = 0;
    let mut upper_bounder: usize = arr.len() - 1;
    let mut idx: usize = 0;

    while lower_bounder <= upper_bounder {
        idx += 1;
        let mid: usize = (upper_bounder + lower_bounder) / 2;
        let mid_value: u32 = arr[mid];

        if mid_value == desired_values {
            return Some((mid_value, mid));
        } else if mid_value < desired_values {
            lower_bounder = mid + 1;
        } else {
            upper_bounder = mid - 1;
        }
    }
    None
}


#[cfg(test)] // run test: cargo test --bin lesson_two or cargo test command if you are in the lesson_two directory
mod test {
    use super::*;
    const ARR: [u32; 15] = [1, 2, 3, 10, 12, 33, 114, 125, 130, 186, 500, 504, 600, 606, 666];

    #[test]
    fn element_found() {
        let target = 600;
        let result = binary_search_example_1(&ARR, target);
        assert_eq!(result, Some(12));
    }
    #[test]
    fn element_not_found() {
        let target = 800;
        let result = binary_search_example_1(&ARR, target);
        assert_eq!(result, None);
    }
}