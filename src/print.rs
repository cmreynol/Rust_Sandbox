pub fn run() {
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Christian", "NC");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Christian", "NC", "code");

    // Named Arguments
    println!("{name} likes to {activity}", name = "Christian", activity = "code");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}