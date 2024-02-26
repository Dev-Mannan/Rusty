//  Module in rust

//  In rust, modules are used to organize code into logical groups. 
//  Modules in rust are created using the module keyword.

mod movies{ //  Module
   pub fn name(){//  Public function
        println!("Animal");
    }
    // priv fn casting(){
    //     println!("Ranbir Kapoor , Anushka Sharma");
    // }

    
}
mod movies_2023{//  Nested module
    pub fn casting(){
        println!("Ranbir Kapoor , Anushka Sharma");
    }
    
}
mod movies_2024{ //  Nested module
    pub mod casting{
        pub fn casting(){
            println!("ShaRukh Khan , Deepika Padukone");
        }
    }
    
}

fn main(){
    
    movies::name();//  Calling the name function
    movies_2023::casting();//  Calling the casting function
    movies_2024::casting::casting();//  Calling the casting function

}