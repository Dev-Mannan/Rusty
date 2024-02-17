use std::io::{self, Write}; // Import Write trait for flush method

fn main() {
    let name: &str = "Mannan";
    println!("My name is {}", name);

    let mut name2 = String::new();
    println!("My name is {}", name2);
    name2.push_str("Mannan");
    println!("My name is {}", name2);

    let name3 = String::from("Mannan");
    println!("My name is {}", name3);

    println!("Enter your Name : ");
    let mut name4 = &string::new();
    io::stdout().flush().expect("Failed to flush"); // Flush stdout
    io::stdin().read_line(&mut name4).expect("Failed to read line");
    println!("Your name is : {}", name4.trim()); // Trim whitespace before printing
}
