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
    let mut numbers = [1, 2, 3, 4, 5];
    let result = utils::loops::fnnd_max_destructuring_the_reference(numbers);
    println!("Result: {:?}", result);
    // let mut numbers = [1, 2, 3, 4, 5];
    // println!("Initial array: {:?}", numbers);
    // let result = utils::loops::mutation_element(&mut numbers);
    // println!("Result: {:?}", result);
}
