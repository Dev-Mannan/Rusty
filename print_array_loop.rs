//  Print array using loop


fn main(){

    let arr = [1,2,3,4,5,6];// array

    // for i in arr.iter(){
    //     println!("{:?}",i);
        
    // }

    for i in 0..6{
        if i==5{
            println!("Array ends here");
            break;
            
        }
        println!("{:?}",arr[i]);

    }

}