//  Structure in Rust 

// Structure is a datatype which is a collection of different data types in rust


struct Employee{ // Declaring a struct
    name:String,
    age:i32,    
    salary:f32
}


fn main(){

    let mut employee1 = Employee{ // Creating a object
        name:String::from("Mannan"),
        age:23,
        salary:3000.0
    };

    println!(" Name : {}",employee1.name);
    println!(" Age : {}",employee1.age);
    println!(" Salary : {}",employee1.salary);

    employee1.age = 24;
    employee1.salary = 4000.0;
    employee1.name = String::from("Mannan Khan");

    println!("-------------------------------------UPDATED VALUE---------------------------------------------"); // Updated value using Mutubality
    println!(" Name : {}",employee1.name);
    println!(" Age : {}",employee1.age);
    println!(" Salary : {}",employee1.salary);
    
    println!("-------------------------------------INPUT Function---------------------------------------------");

    inp(employee1);// Passed the object

}

fn inp(x:Employee){
    println!(" Name : {}",x.name);
    println!(" Age : {}",x.age);
    println!(" Salary : {}",x.salary);
}