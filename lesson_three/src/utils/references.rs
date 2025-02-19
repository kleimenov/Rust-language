pub fn references_integers() -> Option<(i32, i32)> {
    let mut a: i32 = 10;
    let b: i32 = a; // copy operation
    println!("a: {}, b: {}", a, b); 

    a = 20;
    println!("a: {}, b: {}", a, b);
    return Some((a, b)); 
}

pub fn references_strings() {
    let mut s1 = String::from("hello");
    let s2 = s1; // move operation: s1 is moved to s2 (not copy), and then s1 is no longer valid, s1 was reasigned to s2 in heap memory and s1 was invalidated, but s1 variable still exists in scope without a value
    // println!("s1: {}, s2: {}", s1, s2); // This will cause an error because s1 is no longer valid
    s1 = String::from(" world"); // s1 is reasigned to a new value
    println!("s1: {}", s1); // s1 is valid and we can print it
    println!("s2: {}", s2); // still holds the old value of s1, because s2 was assigned to the value of s1 in heap memory and based on exclusive ownership, s2 is still hold s1's old value but not new value, and all further operations on s1 will not affect s2
}