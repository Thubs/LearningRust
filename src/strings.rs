pub fn run() {
    let mut hello = String::from("Hello");

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld");


    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());

    println!("Contains \"World\": {}", hello.contains("World"));

    println!("Replace \"World\" with \"There\": {}", hello.replace("World", "There"));

    println!("{}", hello);
}