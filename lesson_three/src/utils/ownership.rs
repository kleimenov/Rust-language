pub fn ownership() {
    let s = String::from("hello");
    take_ownership(s); // Ownership of s is moved to the function
    // println!("{}", s); // Error: s is no longer valid
}

fn take_ownership(s: String) {
    println!("{}", s);
} // s goes out of scope and is dropped