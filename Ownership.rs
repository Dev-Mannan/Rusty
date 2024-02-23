//  Ownership in Rust

fn main(){

    let v = vec![1,2,3,4,5];
    // println!("{:?}",v);
    // Over here v is a vector which is now owned by x in line 8
    // let x = v;
    // println!("{:?}",x);
    // ve(v);
    // println!("{:?}",);

    let a = ve(v); // in here variable a has now ownership of vector v which was pass to Ve function and in there y has the ownership but it returned
    println!("{:?}",a); // and saveed it now in variable a because a has been storing the ownership of y


}

fn ve(y:Vec<i32>)->Vec<i32>{ // over here y is vector which is now owned by ve function and catching from v -> y -> a 
    y
}