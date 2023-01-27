fn main() {
    // Scalar Types
    //      - Integer types
    
    let x: i8 = 127;
    let y: u8 = 255;
    println!("This is x and y: {x}, {y}");

    let bin = 0b1111_0000;
    let char = b'A';
    println!("This are other numbers: {bin}, {char}");

    //      - Floating point types
    let float = 2.0;
    println!("Another number which is a floating point: {float}");

    // Boolean
    let boolean_var: bool = false;
    println!("This is a boolean: {boolean_var}");

    // Character type
    let char_var: char = 'Z';
    println!("This is a character: {char_var}");


    // Compound Types
    //      - Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Here is a tuple: {:?}", tup);
    let (t1, t2, t3) = tup;
    println!("Its values are: {t1}, {t2}, {t3}");
    println!("Its first value is called tup.0 and holds the value: {}", tup.0);

    //      - Arrays
    let a: [i32; 5] = [3; 5];
    println!("Here is an array, which has fixed length: {:?}", a);
}
