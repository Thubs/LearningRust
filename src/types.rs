pub fn run() {
    let _x = 1;

    let _y = 2.5;

    // Explicit type
    let _z: i64 = 324267890235;

    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("{:?}", (a1, face));
    println!("{:?}", (is_active, _x, _y, _z, is_greater, a1, face));
}