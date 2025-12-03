fn main() {

    // ownership
     let s = String::from("hello");

    takes_ref(&s); // borrow

    println!("{s}"); // works fine

    // taking and returning the ownership
     let s = takes_and_returns(s); // move -> return -> reassign

    println!("{s}");  // works!


    // calculating the length
      let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}
    fn takes_ref(s: &String) {
    println!("Got: {}", s);
}

fn takes_and_returns(s: String) -> String {
    println!("Inside function: {s}");
    s   // return ownership back
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}