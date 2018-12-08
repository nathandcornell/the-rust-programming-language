fn main() {
    if_expressions();
    loops();
    while_loops();
    for_loops();
}

fn if_expressions() {
    let three = 3;
    let five = 5;

    if three < five {
        println!("Three is less than Five!");
    } else {
        println!("Three is NOT less than Five!");
    }

    // Compiler error!
    // "expected bool, found &str"
    /*
        let true_string = "true";
        if true_string {
            println!("True");
        }
    */

    let numerator = 12;

    if numerator % 2 == 0 {
        println!("{} is divisible by 2", numerator);
    }
    else if numerator % 3 == 0 {
        println!("{} is divisible by 3", numerator);
    } else {
        println!("{} is neither divisible by 3, nor 2", numerator);
    }

    let maybe_five = if true {
        5
    } else {
        0
    };

    println!("The value of maybe_five is: {}", maybe_five)
}

fn loops() {
    // Infinite loop!
    /*
        println!("It just keeps going");
        loop {
            println!("and going");
        }
     */

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn while_loops() {
    let mut countdown = 3;

    while countdown > 0 {
        println!("{}", countdown);
        countdown -= 1;
    }

    println!("BOOM!");
}

fn for_loops() {
    let fruits = ["apple", "banana", "watermelon", "orange"];

    for fruit in fruits.iter() {
        println!("My favorite fruit is {}", fruit);
    }

    for digit in (1..=5).rev() {
        println!("{}...", digit);
    }

    println!("BLAST OFF!!")
}
