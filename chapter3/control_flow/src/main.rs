fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else if number % 2 == 0 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = false;
    let x = if condition { 5 } else { 6 }; 
    println!("The value of x is {x}");

    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Disambiguate
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Loops through collections
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!");
}
