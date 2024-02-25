//  Concurrency in rust 

//  Concurrency is the ability of a program to perform multiple tasks simultaneously. 
//  Concurrency in rust is achieved through the use of threads.

//  In rust, threads are created using the thread::spawn() function.

use::std::thread;
use::std::time::Duration;
fn main() {
    //  A thread is created using the thread::spawn() function

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        };
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))}



}