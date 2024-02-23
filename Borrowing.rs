//  Borrowing in Rust

// When a fuction transfer it's contrl over a variable / value to another function temporaryly and return it from which it borrowed


fn _custom(s:&Vec<i32>){

    println!("{:?}",s);
    

}






fn main(){
    let a = vec![1,2,3,4,5];

    println!("{:?}",a);

    _custom(&a);

    println!("{:?}",a);

}