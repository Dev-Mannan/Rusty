//Naming & keyword function in rust

fn main(){
    
    let _name = "DevMannan";//Naming convention
    println!("Hello, {}", _name);

    let _age = 23; // before _ cant use digits as well cant use Capital letter as it's a Snake case

    println!("Age: {}", _age);

    
            //Keywords  =  are the special words in rust

        let kname: [&str; 7] = ["fn", "let", "return", "if", "else", "while", "for"];
    
        for i in &kname {
            println!("{}", i);
        }

    
}