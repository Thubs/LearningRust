// The length of arrays are fixed

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [ 2, 3, 4, 5];

    // Re-sign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Signle value: {}", numbers[0]);

    println!("Array length: {}", numbers.len());

    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}