pub fn borrowing_general_concept() {
    let mut s1 = String::from("hello");
    let mut s2 = &s1; // s2 borrows s1, s2 is a reference to s1
    println!("s1: {}, s2: {}", s1, s2);

    demo(& mut s1); // s1 is borrowed by demo function and we can modifi string s1 in demo function because we passed a mutable reference to s1
    println!("s1 after demo function execution: {}", s1);    
}

fn demo(s3: & mut String) {
    s3.push_str(" world");
}