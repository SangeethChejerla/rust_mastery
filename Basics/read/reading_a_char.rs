use std::io;

fn main() {
    println!("Enter a single character:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let first_char = input.trim().chars().next().unwrap_or('\0');
    println!("You entered: {}", first_char);
}
