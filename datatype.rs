// Datatypes in rust

fn main(){
    //types of Datatypes

    // 1. Scalar Datatypes
    let number = 10; // Integer
    let decimal = 10.0; // Float
    let name = "DevMannan"; // String
    let flag = true; // Boolean
    let alpha = 'a'; // Character

    println!("Number: {}, Decimal: {}, Name: {}, Flag: {}, Alpha: {}", number, decimal, name, flag, alpha);

    if flag == true {
        println!("The Flag you have declared is true");
        
    }else{
        println!("The Flag you have declared is false");
    }
}