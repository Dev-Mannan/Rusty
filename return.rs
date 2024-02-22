// Return statement in rust


fn main(){

    println!("return Function in rust");
    println!("{}",return_fun(23,2));

}
   // over here -> return_fun() a integer value
fn return_fun(x:i32,y:i32)->i32{ //return value denotes -> i32 type
    return x+y;
}