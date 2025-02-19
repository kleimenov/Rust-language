/* Lesson Three */
// This lesson will cover the following topics:
// 1. Ownership
// 2. Borrowing
// 3. Stack and Heap
// 4. References

mod utils;

fn main() {
    println!("Lesson Three");
    println!("EXPERIMENT 1 ownership strings: ---------------------------");
    utils::ownership::ownership();

    println!("EXPERIMENT 3 borrowing general concept: ---------------------------");
    utils::borrowing::borrowing_general_concept();
    println!("EXPERIMENT 4 references integers: ---------------------------");
    let result = utils::references::references_integers();
    println!("{:?}", result);
    println!("EXPERIMENT 4 references strings: ---------------------------");
    utils::references::references_strings();
}
