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

    // ===========================

    let s8 = String::from("Mama niet!!!");
    let len = calculate_length_with_reference(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("Dawajcie!");
    let sth = &mut s9;
    // let sth2 = &mut s9; // Nie można pożyczać więcej niż jeden raz!
    change_mutable_reference(sth);
    // println!("{}, {}", sth, sth2);

    // --------------------------

    let mut s10 = String::from("Illegal");
    let ref1 = &s10;
    let ref2 = &s10;
    // let ref3 = &mut s10; // Mutable ref not possible, overlap
    println!("{} :: {}", ref1, ref2);
    let ref3 = &mut s10; // Mutable ref possible, no overlap
    println!("{}", ref3);

    // --------------------------

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    let s11 = String::from("Hello world");
    let s12 = first_word(&s11);
    println!("{}", s12);
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

fn calculate_length_with_reference(s: &String) -> usize {
    s.len()
}

fn change_mutable_reference(s: &mut String) {
    s.push_str("please");
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

fn dangle() -> String {
    let s = String::from("hello from hell");
    s
}

// --------------------------

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}