fn main() {
    let s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let s3 = String::from("Kusy");
    takes_ownership(s3);

    let i1 = 5;
    makes_copy(i1);

    // tests_existence(s3); // Nie zadziała! Bo jest metoda Drop!
    tests_existence(i1); // Zadziała! Bo jest metoda Copy!

    // --------------------------

    let s4 = gives_ownership();
    let s5 = String::from("Hi!");
    let s6 = takes_and_gives_back(s5);

    println!("{}, {}", s4, s6);

    // --------------------------

    let s7 = String::from("Iordan");

    let (s8, len) = calculate_length(s7);
    println!("The length of '{}' is {}.", s8, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn tests_existence(some_value: i32) {
    println!("{}", some_value);
}

// --------------------------

fn gives_ownership() -> String {
    let s = String::from("Yours");
    s
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

// --------------------------

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}