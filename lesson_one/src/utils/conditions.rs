// control flow statements in Rust: if, else, else if, match
pub fn if_statment() -> i32 {
    let mut a: i32 = -5;
    let mut boolean = true;
    // how to check if number true or false
    if a > 0 {
        println!("condition was true");
    }
    a // return a or we can use return word: return a

} 

pub fn if_else_statment() {
    let a: i32 = -5;
    if a > 0 {
        println!("variable a greater than zero");
    } else if a < 0 {
        println!("variable a less than zero");
    } else {
        println!("variable a equal to zero");
    }
}