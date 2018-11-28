fn main() {
    // Variables and Mutability
    let five = 5;
    println!("The value of five is: {}", five);
    // five = 6; // Compiler error! "cannot assign twice to immutable variable"
    // println!("The value of five is now: {}", five);
    
    let mut six = 6;
    println!("The value of six is: {}", six);
    six = 9; // Hendrix
    println!("The value of six is now: {}", six);

    // Constants
    const DAYS_OF_SUMMER: u32 = 100;
    println!("{} Days of Summer", DAYS_OF_SUMMER);

    // Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    println!("The value of 'x' is: {}", x);

    let spaces = "  ";
    let spaces = spaces.len();

    let space_count = "  ".len();

    println!("spaces = {}, and space_count = {}", spaces, space_count);

    let mut spacey = "  ";
    // spacey = spacey.len(); // Compiler error! "expected &str, found usize"
}
