pub fn run() {
    let mut hello = String::from("Hello");

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld");

    println!("Capacity: {}", hello.capacity());  

    println!("Is Empty: {}", hello.is_empty());

    println!("Contains \"World\": {}", hello.contains("World"));

    println!("Replace \"World\" with \"There\": {}", hello.replace("World", "There"));

    // For loop
    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);
}