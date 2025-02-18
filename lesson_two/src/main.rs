// type of loops
mod utils;

fn main() {
    // utils::loops::_loop();
    // println!("---------------------------------");
    // utils::loops::_while();
    // println!("---------------------------------");
    // utils::loops::_for_loop();
    // println!("---------------------------------");
    // utils::loops::_for_loop_II();
    // println!("---------------------------------");
    // let mut numbers = [1, 2, 3, 4, 5];
    // let result = utils::loops::fnnd_max_destructuring_the_reference(numbers);
    // println!("Result: {:?}", result);
    // let mut numbers = [1, 2, 3, 4, 5];
    // println!("Initial array: {:?}", numbers);
    // let result = utils::loops::mutation_element(&mut numbers);
    // println!("Result: {:?}", result);
    let numbers: [i32; 15]= [1, 2, 3, 10, 12, 33, 114, 125, 130, 186, 500, 504, 600, 606, 666];
    let target = 2;

    let result = utils::binary_search_usses_much::binary_search(&numbers, target);

    match result {
        Some((found_value, index)) => println!("Target {} found at index {}", found_value, index),
        None => println!("Target {} not found in the array", target),
    }
}

