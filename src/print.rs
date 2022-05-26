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
}