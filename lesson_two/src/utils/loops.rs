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
    let array_of_numbers: [u32; 5] = [1, 2, 3, 4, 5];

    for number in array_of_numbers {
        println!("This is a for loop {}", number);
    }
}

pub fn find_max(numbers: [u32; 5]) -> u32 {
    let mut max = numbers[0];
    for number in numbers.iter() {
        if *number > max { // dereference the number to get the value of the number in the array (actual value)
            max = *number; // assign the value of the number to max
        }
    }
    max
}

pub fn fnnd_max_destructuring_the_reference(numbers: [u32; 5]) -> u32 {
    let mut max = numbers[0];
    for &number in numbers.iter() {
        if number > max { // destructuring the reference to get the value of the number in the array (actual value)
            max = number; // assign the value of the number to max
        }
    }
    max
}

pub fn mutation_element(numbers: &mut [u32; 5]) -> &mut [u32; 5] {
    for number in numbers.iter_mut() {
        *number = *number + 1;
    }
    numbers
}