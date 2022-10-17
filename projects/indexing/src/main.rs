use std::io;

fn main() {
    let arr: [i32; 5] = [1, 2, 4, 8, 16];
    
    let mut idx = String::new();

    println!("Array: {:?}", arr);
    println!("Enter an array index:");

    io::stdin().read_line(&mut idx).expect("Failed to read user input");

    let idx: usize = match idx.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Cannot parse to integer")
    };

    println!("Index: {idx}");
    println!("Array Element at index: {}", arr[idx]);
    println!("Array length: {}", arr.len());
}
