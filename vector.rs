// Vector in Rust 


fn main (){
    let mut age = vec![23,24,25,26,27];
    println!("{}",age[0]);
    age.push(29);
    for i in age{
        println!("{}",i);
    }

    let mut v:Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    println!("{:?}",v[1]);
}