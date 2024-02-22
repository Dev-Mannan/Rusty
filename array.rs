// array in Rust


fn main(){

    // declare array
    let arr = [1,2,3,4,5,7,7,8,9,10];
    
    println!("{:?}",arr);    // place holder in array can be use in tuple because it is a compound datatype

    println!("--------------------------");

    let mut arr1: [i32; 5] = [1,4,2,3,5];
    arr1.sort();
    println!("{:?}",arr1);

    let mut arr2: [i32; 10] = [100;10]; //default value can be use in rust
    arr1.sort();
    println!("{:?}",arr2);

    let mut arr3: [i32; 5] = [1,4,2,3,5];
    arr3[5] =13;
    println!("{:?}",arr3);




}