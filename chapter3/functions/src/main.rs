fn main() {
    println!("Hello, world!");
    
    another_function(-67, 'B');


    // Expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is {x}");
}

fn another_function(x: i32, label: char) {
    println!("The values are: {x} and {label}");
}

fn five() -> i32 {
    5
}
