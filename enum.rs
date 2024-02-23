//  Custom datatype in Rust

#[derive(Debug)]
enum Gender{
    Male,
    Female,
    
}


    struct Person{
        
        name:String,
        gender:Gender,

        
    }

fn main(){

        let x = Gender::Male;
        let y = Gender::Female;

    

    let z = Person{
        name:String::from("Mannan"),
        gender:Gender::Male
    };
    println!("Name : {}",z.name);
    println!("Gender  : {}",z.gender);


}