//  Function in Rust 

fn main() {
    println!("This is Main Function");
    greet();
    println!("{}",add(1,2));
    println!("{}",sub(1,2));
    println!("{}",multiply(1,2));

}
fn greet(){
    println!("Simple Functional calculation");
}

fn add(x:i32,y:i32)->i32{
    return x+y;
}
fn sub(x:i32,y:i32)->i32{
    return x-y;
}
fn multiply(x:i32,y:i32)->i32{
    return x*y;
}