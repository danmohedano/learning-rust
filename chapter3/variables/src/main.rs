const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    println!("The value of the constant is {THREE_HOURS_IN_SECONDS}");


    // Shadowing vs mutable
    let z = "     ";
    let z = z.len();

    println!("The user wants {z} spaces in their input.");
}
