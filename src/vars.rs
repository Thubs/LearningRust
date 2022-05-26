pub fn run() {
    let name = "Thubs";
    let mut age = 15;

    println!("My age is {}", age);
    age = 16;
    println!("My name is {0}, and I am {1} years old", name, age);

    // Define constant
    const ID: i32 = 101;
    println!("ID: {}", ID);

    // Assign multiple vars
    let ( my_name, my_age ) = ("Thubs", 15);
    println!("{} is {}", my_name, my_age);
}