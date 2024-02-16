// Literals and Typecasts
// https://doc.rust-lang.org/book/ch03-02-data-types.html


fn main(){
    let one = 1;
    let two:i8 = 2;
    let three = 3i8; // This is a literal
    println!("----------------  Literals  ----------------");

    println!("{} {} {}", one, two, three);

    println!("----------------  Typecasts  ----------------");

    let five = 5;
    println!("{} -Byte",std::mem::size_of_val(&five));
    let six = 6i8;
    println!("{} -Byte",std::mem::size_of_val(&six));
    let seven = five as i8;
    println!("{} -Byte",std::mem::size_of_val(&seven));
}