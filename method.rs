// Methods in Rust
// Methods are a function for a specific structure is called methods in rust

struct Rectangle{ // Declaring a struct
    breadth:f32,
    length:f32
}

impl Rectangle{ // Implementing methods using impl Keyword
    fn area(&self)->f32{          //  &self is a reference , fn used for method
        self.breadth*self.length
    }
    fn perimeter(&self)->f32{    // another usecase of this perticular method
        self.breadth*2.0 + self.length*2.0
    }
}

fn main(){
    let rect = Rectangle{breadth:20.0,length:30.0}; // object creation for struct
    println!("____________________Area of Rectangle_____________________________");
    println!("Area of Rectangle : {}",rect.area()); // accessing area method
    println!("____________________Perimeter of Rectangle_____________________________");
    println!("Perimeter of Rectangle : {}",rect.perimeter()); // accessing perimeter method
}