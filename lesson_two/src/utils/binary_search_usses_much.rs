use std::cmp::Ordering;

pub fn binary_search(arr: &[i32], target: i32) -> Option<(i32, usize)> {
    println!("Binary search uses much space for iterative implementations.");

    let mut lower_bound = 0; // Start of the array will reference the first element and consider as lower bound
    let mut upper_bound = arr.len() - 1; // End of the array will reference the last element and consider as upper bound
    let mut idx: usize = 0;

    while lower_bound <= upper_bound {
        idx += 1;
        let mid: usize = (upper_bound + lower_bound) / 2; // Find the middle element of the array by adding lower bound and upper bound and divide by 2
        let mid_value: i32 = arr[mid];

        println!("Iteration: {}", idx);
        match mid_value.cmp(&target) {
            Ordering::Equal => return Some((mid_value, mid)),
            Ordering::Less => lower_bound = mid + 1,
            Ordering::Greater => upper_bound = mid - 1,
        }
    }
    None
}

#[cfg(test)] // run test: cargo test --bin lesson_two or cargo test command if you are in the lesson_two directory
mod test {
    use super::*;
    const ARR: [i32; 15] = [-1, 2, 3, 10, 12, 33, 114, 125, 130, 186, 500, 504, 600, 606, 666];

    #[test]
    fn smallest_element_not_found() {
        let target = -11;
        let result = binary_search(&ARR, target);
        assert!(result.is_none());
    }
}