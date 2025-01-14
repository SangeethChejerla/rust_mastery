use std::io;

fn main() {
    println!("Enter an integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please type a valid integer");
    println!("You entered: {}", number);
}
