// Datatypes in rust

fn main(){
    //types of Datatypes

    // 1. Scalar Datatypes
    let int_num :i8 = -128; // in this datatype where i8 is a 8 bit integer where either it will hold till positive 128 or negative 128
    let int_num2:i16 = 32000;
    let int_num3:i32 = 320000000;
    let int_num4:i64 = -3200000000000000000;
    let int_num5:i128 = -32000000000000000000000000000000000000;

    let int_num6:u8 = 128; // in this datatype where u8 is a 8 bit integer where either it will hold till positive  128
    let int_num7:u16 = 32000;
    let int_num8:u32 = 320000000;
    let int_num9:u64 = 3200000000000000000;
    let int_num10:u128 =32000000000000000000000000000000000000;

    

    println!("-----------------------------------------Signed----------------------------------------------");
    println!("int_num: {}", int_num);
    println!("int_num2: {}", int_num2);
    println!("int_num3: {}", int_num3);
    println!("int_num4: {}", int_num4);
    println!("int_num5: {}", int_num5);
    //Usnsigned datatypes only positive number
    println!("-----------------------------------------Unsigned----------------------------------------------");
    println!("int_num: {}", int_num6);
    println!("int_num2: {}", int_num7);
    println!("int_num3: {}", int_num8);
    println!("int_num4: {}", int_num9);
    println!("int_num5: {}", int_num10);
}