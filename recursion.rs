//  Recursion in rust

//  A function that calls itself is called recursion in rust . Recursive function is a function that calls itself.

fn factorial(n: i32) -> i32 {
    if n > 1 {
        n*factorial(n-1)
    }
    else {
        n
    }
}


fn main() {
    println!("{}", factorial(9));
}