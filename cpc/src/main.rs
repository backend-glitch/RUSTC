mod ex_1;

fn main() {
// cpc => common programming concepts

// constant
const NUM: i32 = 5;
println!("the constant : {NUM}");

//immutable variable
let x = 10;
println!("the immutable number : {x}");

//mutable
let mut y = 35;
y += 1;
println!("the mutable number : {y}");

// shadowing 
let spaces = " ";
let spaces = spaces.len(); // converted to number

// u32
let guess: u32 = "42".parse().expect("not a number");

//boolean
let t = true;
println!("{t}");

//tuples
let tup = (500,6.4,1);
let (x,y,z) = tup;
println!("the value of: {y}");

//array
let nums: [i32,5] = [1,2,3,4,5];
println!("{:?}",nums);
//let second  = nums[1];

//importing function
ex_1::another1();

  let jk = five();
  println!("The value of x is: {jk}");
}

// function
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

//returning function
fn five() -> i32 {
    5
}
    // with parameters
fn plus_one(x: i32) -> i32 {
    x + 1;
}
