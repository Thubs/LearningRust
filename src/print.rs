pub fn run() {
    // Print to console
    println!("Hello, world!");

    // Basic formatting
    println!("{} days", 31);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");

    // Placeholder traits
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 x 10 = {}", 10 / 10);
}