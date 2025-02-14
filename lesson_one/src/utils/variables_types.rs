// Variables types in Rust

pub fn var_primitive_types() {
    println!("Primitive variables types in Rust");
    // Integer types
    let x_i8: i8 = 127; // 8-bit signed integer type, ranges from -128 to 127 (can be negative or positive)
    let x_u8: u8 = 127; // 8-bit unsigned integer type, ranges from 0 to 255 (only positive)
    println!("--------------- Integer types ");
    println!("Integer types i8: {}", x_i8);
    println!("Integer types u8: {}", x_u8);

    // Floating-point types
    let x_f32: f32 = 3.14; // 32-bit floating-point type
    let x_f64: f64 = 3.14; // 64-bit floating-point type

    println!("--------------- Floating-point types ");
    println!("Floating-point types f32: {}", x_f32);
    println!("Floating-point types f64: {}", x_f64);

    // Boolean type
    let _bool_true: bool = true; // Boolean type, either true or false
    let _bool_false: bool = false; // Boolean type, either true or false

    println!("--------------- Boolean type ");
    println!("Boolean type true: {}", _bool_true);
    println!("Boolean type false: {}", _bool_false);

    // Character type
    let letter: char = 'A'; // Character type, represents a single Unicode scalar value

    println!("--------------- Character type ");
    println!("Character type: {}", letter);

    // String type
    let text: &str = "Hello, World!"; // String type, a sequence of Unicode scalar values

    println!("--------------- String type ");
    println!("String type: {}", text);

    // Unit type
    let _unit: () = (); // Unit type, represents an empty tuple
    println!("--------------- Unit type ");
    println!("Unit type: {:?}", _unit);
}

pub fn var_composite_types() {
    println!("Composite variables types in Rust");
    // Array type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array type, a fixed-size collection of elements of the same type
    println!("--------------- Array type ");
    println!("Array type: {:?}", numbers);
    let array_of_tuples: [(i32, &str); 2] = [(1, "one"), (2, "two")]; // Array type, a fixed-size collection of elements of the same type
    println!("Array type: {:?}", array_of_tuples);

    // Tuple type
    let person: (&str, i32, &str) = ("John", 30, "New York"); // Tuple type, a collection of elements of different types
    println!("--------------- Tuple type ");
    println!("Tuple type: {:?}", person);

    // Slice type
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // Array type
    let slice: &[i32] = &numbers[1..3]; // Slice type, a dynamically-sized view into a contiguous sequence
    let slice_1: &[i32] = &numbers[0..1]; // Slice type, a dynamically-sized view into a contiguous sequence
    println!("--------------- Slice type ");
    println!("Slice type: {:?}", slice); // [2, 3] - prints the elements from index 1 to 3
    println!("Slice type: {:?}", slice_1); // [1] - prints the elements from index 0 to 1
    println!("Slice type: {:?}", numbers[0]); // 1 - prints the element at index 0
}

pub fn var_collections_types() {
    println!("Collections variables types in Rust");
    // Vector type
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // Vector type, a dynamically-sized collection of elements of the same type
    numbers.push(6);
    numbers.push(7);

    println!("--------------- Vector type ");
    println!("Vector type: {:?}", numbers);

    // HashMap type
    use std::collections::HashMap;

    let mut scores = HashMap::new(); // HashMap type, a collection of key-value pairs
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);

    println!("--------------- HashMap type ");
    println!("HashMap type: {:?}", scores);

    // HashSet type
    use std::collections::HashSet;

    let mut colors = HashSet::new(); // HashSet type, a collection of unique values
    colors.insert(String::from("Red"));
    colors.insert(String::from("Blue"));
    colors.insert(String::from("Red"));

    println!("--------------- HashSet type ");
    println!("HashSet type: {:?}", colors);

    // BTreeMap type
    use std::collections::BTreeMap;

    let mut scores = BTreeMap::new(); // BTreeMap type, a collection of key-value pairs sorted by key
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 20);

    println!("--------------- BTreeMap type ");
    println!("BTreeMap type: {:?}", scores);

    // BTreeSet type
    use std::collections::BTreeSet;

    let mut colors = BTreeSet::new(); // BTreeSet type, a collection of unique values sorted in ascending order
    colors.insert(String::from("Red"));
    colors.insert(String::from("Blue"));
    colors.insert(String::from("Red"));

    println!("--------------- BTreeSet type ");
    println!("BTreeSet type: {:?}", colors);

    // LinkedList type
    use std::collections::LinkedList;

    let mut list = LinkedList::new(); // LinkedList type, a doubly linked list
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    println!("--------------- LinkedList type ");
    println!("LinkedList type: {:?}", list);

    // BinaryHeap type
    use std::collections::BinaryHeap;

    let mut heap = BinaryHeap::new(); // BinaryHeap type, a priority queue
    heap.push(1);

    println!("--------------- BinaryHeap type ");
    println!("BinaryHeap type: {:?}", heap);
}

pub fn _info() {
    // function return nothing
    println!("PLACEHOLDER");
}