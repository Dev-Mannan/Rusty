

fn main() {
    let name: &str = "Mannan";
    println!("My name is {}", name);

    let mut name2 = String::new();
    println!("My name is {}", name2);
    name2.push_str("Mannan");
    println!("My name is {}", name2);
    // String slicing
    let show = &name2[0..4];
    println!("My name is {}", show);

    let name3 = String::from("Mannan");
    println!("My name is {}", name3);
//   Concatenation of two or more strings
    println!("{}+{}+{}",name, name2, name3);
}
