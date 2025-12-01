use std::io; // to take the input 
use rand; // rand::Rng => to generate a random number
use std::cmp::Ordering; // to compare the numbers

fn main() {
 
 println!("Guess the number game");

 let secret_number = rand::random_range(1..=100);
  // rand::thread_rng => to generate a random number
 // gen_range(start--=end) => to specify the range.

 loop{

 println!("Enter the number");
 
 let mut guess = String::new(); // to create a new string reference variable (mutable)

 
 io::stdin()         // to access keyboard input
    .read_line(&mut guess)  // to store user input in guess variable
    .expect("failed to read a line"); // to handle exception cases

let guess : u32 = match guess.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
};
// u32 = number type // parse => to convert strings
// parse returns result which is a enum and hase variant (ok and err).

println!("your guessed number : {guess}");

match guess.cmp(&secret_number){ // to compare , match is like switch in c++
    Ordering::Less => println!("too small!"),
     Ordering::Greater => println!("too bigl!"),
      Ordering::Equal =>{ 
        println!("you win!");
        break; // break program ends
        // _ => to represent default in the match()
        }
       }
//println!("The secret number :{secret_number}");
     }
    }
