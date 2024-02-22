// User input in rust

use std::io;

fn main(){

    
    let mut input = String::new();
    println!("Enter your name : ");
    let _b1 = io::stdin().read_line(&mut input).unwrap();
    println!("Your name is : {}",input);

}