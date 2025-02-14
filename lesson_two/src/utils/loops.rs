// Rust provides several types of loops: loop, while, and for.

pub fn _loop() {
    println!("This is a loop function");
    // loop is an infinite loop (if you want to break the loop, you can use break keyword)
    let mut cnt: i32 = 0;
    // infinit loop example
    // loop {
    //     cnt = cnt + 1;
    //     println!("This is an infinite loop {}", cnt);
        
    // }

    loop {
        cnt = cnt + 1;
        println!("This is loop {}", cnt);
        if cnt == 5 {
            break;
        }
    }
}  

pub fn _while() {
    let mut cnt: i32 = 0;
    while cnt < 5 {
        println!("This is a while loop {}", cnt);
        cnt = cnt + 1;
    }
}

pub fn _for_loop() {
    // for loop
    let array_of_numbers: [i32; 5] = [1, 2, 3, 4, 5];

    for number in array_of_numbers {
        println!("This is a for loop {}", number);
    }
}

pub fn _for_loop_II() {
    // for loop
    let array_of_numbers = [10, 12, 23, 34, 85];

    for (index, value) in array_of_numbers.iter().enumerate() {
        println!("This is a for loop II index {}, value {}", index, value);
    }
}