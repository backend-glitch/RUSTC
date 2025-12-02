// accessing array elements

use std::io;

fn another1(){

    let a = [1,2,3,4,5];

    println!("Enter the array index");
    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("failed!");

    let index: usize = index.trim().parse().expect("invalid input ,not a number");

    let element = a[index];
    println!("the value of this {index} is : {element}");
}