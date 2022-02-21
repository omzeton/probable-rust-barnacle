use std::io;

fn main() {
    println!("Is the temperature in Fahrenheit?");

    let mut is_fahrenheit = String::new();

    io::stdin()
        .read_line(&mut is_fahrenheit)
        .expect("Please type either \"yes\" or \"no\"");
    
    let is_fahrenheit: bool = match is_fahrenheit.trim().parse::<String>() {
        Ok(answer) => answer == "yes",
        Err(_) => false,
    };

    println!("Provide the temperature degree");

    let mut degree = String::new();

    io::stdin()
        .read_line(&mut degree)
        .expect("Please input a correct value");
    
    let degree: i32 = match degree.trim().parse() {
        Ok(answer) => answer,
        Err(_) => 0,
    };

    if is_fahrenheit {
        let answer: i32 = (degree - 32) * 5/9;
        println!("{}°F is {}°C", degree, answer);
    } else {
        let answer: i32 = (degree * 5/9) + 32;
        println!("{}°C is {}°F", degree, answer);
    }
}
