fn main() {
    let mut s = String::from("hello");
    println!("The program says {s}!");

    s.push_str(" there");
    println!("The string has been modified to {s}");
    
    // Shallow copy == move in Rust (s1 is invalidated)
    //let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{}, world!", s1);
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    // Ownership and functions
    let s = String::from("hello");
    takes_ownership(s);
    println!("s can't be used after function call because its value was borrowed");

    let x = 5;
    makes_copy(x);
    println!("This is x after function call {x}");

    // Return values and scopes
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{s1}, {s3}");

    // References
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");

    // Mutable references
    let mut s = String::from("hello");
    println!("Before change: {s}");
    change(&mut s);
    println!("After change: {s}");

    // Slices
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("The first word of '{s}' is '{word}'");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
