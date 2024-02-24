

#[derive(Debug)]
enum H{
    Named(String),
    Age(i32),
}

#[derive(Debug)]
enum Vehicle{
    Car,
    Bike,
}

fn fn_for_enum(x:Vehicle){
    
    match x {
        Vehicle::Car => println!("Range Rover"),
        Vehicle::Bike => println!("Yamaha"),

    }
}

fn main(){

    let  set_name = H::Named(String::from("John"));
    let  set_age = H::Age(30);

    println!("{:?}", set_name);
    println!("{:?}", set_age);

    fn_for_enum(Vehicle::Bike);
    fn_for_enum(Vehicle::Car);

}