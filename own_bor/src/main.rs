fn main() {

    // ownership
     let s = String::from("hello");

    takes_ref(&s); // borrow

    println!("{s}"); // works fine

    // taking and returning the ownership
     let s = takes_and_returns(s); // move -> return -> reassign

    println!("{s}");  // works!


    // calculating the length ,taking the ownership
      let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    //this time not by ownership but by reference
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");


    // like variables , reference are immutable by default


    // there can be many immutable references but only one mutable reference that also when immutable are out os scope!!
    let mut s4 = String::from("hello");

    let r1 = &s4; // no problem
    let r2 = &s4; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s4; // no problem
    println!("{r3}");


    // dangling pointer
    let strings = no_dangle();

    //slices
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];

//slices on arrays
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
}
    fn takes_ref(s: &String) {
    println!("Got: {}", s);
}

fn takes_and_returns(s: String) -> String {
    println!("Inside function: {s}");
    s   // return ownership back
}

// taking ownership
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// taking reference
fn calculate_length(s: &String) -> usize {
    s.len()
}


fn no_dangle() -> String {
    let s = String::from("hello");

    s
}