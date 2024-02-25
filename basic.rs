
use::std::convert::TryInto;

fn add(x:u128){
    println!("{}",x);
}
fn main()
{
    add(100000005u128.try_into().unwrap());
}