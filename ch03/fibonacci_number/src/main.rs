use std::io;

fn main() {
    let fibonacci_digit:i32 = get_digit();
    let nth_fibonacci_digit:i32 = calculate_nth_digit(fibonacci_digit);

    println!(
        "Fibonacci number #{} is {}",
        fibonacci_digit,
        nth_fibonacci_digit
    );
}

fn get_digit() -> i32 {
    println!("Which digit of the Fibonacci Sequence should we return?");

    loop {
        let mut digit = String::new();

        io::stdin().read_line(&mut digit)
            .expect("Failed to read line");

        let digit: i32 = match digit.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter an integer value like '3' or '28'");
                continue;
            },
        };

        return digit;
    }
}

fn calculate_nth_digit(digit_to_return: i32) -> i32 {
    return fib(0, 1, 0, digit_to_return);
}

fn fib(current_digit: i32, last_digit: i32, count: i32, digit_to_return: i32) -> i32 {
    if count == digit_to_return {
        return current_digit;
    }

    return fib(current_digit + last_digit, current_digit, count + 1, digit_to_return);
}
