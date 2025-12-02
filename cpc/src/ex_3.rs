// fibonacci series
fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}
/*
fn fibonacci(n: u32) -> u64 {

    if n <= 1 {
        return n as u64;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

*/

fn main() {
    let n = 10;
    println!("Fibonacci({}) = {}", n, fibonacci(n));
}
