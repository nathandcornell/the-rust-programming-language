fn main() {
    /* Statements */
    let x = 5;
    let y = 6;

    // **Statements do _not_ return a value**
    // Compiler error! "expected expression, found statement (`let`)"
    // let a = (let b = 4);

    // Compiler error! "cannot find value `d` in this scope"
    // let c = d = 4;
    
    /* Expressions */
    let z = {
        let a = 2;
        a * 5
    };

    println!("The value of z is: {}", z);

    // This doesn't work, because the block returns nothing:
    /*
        let z = {
            let a = 2;
            a * 5; // This semicolon makes it a statement
        };
    */


    /* Functions */
    print_value(x);
    print_two_values(x, y);

    let a = five();
    println!("The value of a is: {}", a);

    let b = five_plus_one();
    println!("The value of b is: {}", b);

    // let c = five_plus_two();
    // println!("The value of c is: {}", c);
}

/* Functions */
fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_two_values(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn five_plus_one() -> i32 {
    let x = five();
    x + 1
}

// Compiler error!
// "mismatched types"
/*
    fn five_plus_two() -> i32 {
        let x = five();
        x + 2; // This semicolon prevents a return value
    }
*/
