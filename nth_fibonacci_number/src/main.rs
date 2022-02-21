use std::io;

fn fib(index: u32) -> u32 {
    if index <= 1 {
        index
    } else {
        fib(index - 1) + fib(index - 2)
    }
}

fn main() {
    println!("Select index of the desired Fibonacci number.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Please try again.");
    
    let index: u32 = match index.trim().parse::<u32>() {
        Ok(answer) => answer,
        Err(_) => 0,
    };

    println!("{}", fib(index));
}
