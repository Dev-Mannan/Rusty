//  closures in Rust 





fn main(){
 
//   Declare a closure in a Variable 
    let x = | i:i32| -> i32 {i*10};

    println!("{}", x(10));
//  Differnet way of declaring a closure in a variable in Rust
    let y = | i | {i*100};
    println!("{}", y(100));


}