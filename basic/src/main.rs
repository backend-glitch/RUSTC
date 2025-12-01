fn main() {
    println!("Hello, world!");

    let x = 5;       // immutable
    let mut y = 10;  // mutable
    const PI: f64 = 3.1415;  // constant
    
    y+=1;

    println!("{x}, {y}");
    println!(" {PI}");


    let arr = [1, 2, 3];
    let mut v = vec![1, 2, 3];
    v.push(4);  // add element

    let s = String::from("hello");
    let part = &s[0..3];  // slice
    println!("{part},the array:{:?}",arr);


}
