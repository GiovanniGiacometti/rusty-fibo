use std::io;

fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("Insert n:");
    
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i32 = n.trim().parse().expect("Wtf!");

    println!("Fibonacci of number {} is {} ",  n, fibonacci(n));
}
