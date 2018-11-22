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
}
