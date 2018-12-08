use std::io;

fn main() {
    print_banner();

    let temperature_scale:char = get_temperature_scale();
    println!("The temperature_scale is {}", temperature_scale);

    let temperature_to_convert = get_temperature_to_convert();
    println!("The temperature_to_convert is {}", temperature_to_convert);

    let new_scale = get_new_scale(temperature_scale);

    let new_temperature = if new_scale == 'F' {
        c_to_f(temperature_to_convert)
    } else {
        f_to_c(temperature_to_convert)
    };

    println!(
        "{}{} is {}{}",
        temperature_to_convert,
        temperature_scale,
        new_temperature,
        new_scale
    );
}

fn get_temperature_scale() -> char {
    println!("What is your starting temperature scale?");

    loop {
        println!("(Enter 'C' for Celsius, or 'F' for Fahrenheit):");

        let mut temperature_scale = String::new();
        
        io::stdin().read_line(&mut temperature_scale)
            .expect("Failed to read line");

        let temperature_scale: char = match temperature_scale.trim().parse() {
            Ok(character) => character,
            Err(_) => {
                println!("Please enter 'C' or 'F'");
                continue;
            },
        };

        if temperature_scale == 'C' || temperature_scale == 'F' {
            return temperature_scale;
        }

        println!("Please enter 'C' or 'F'");
    }
}

fn get_temperature_to_convert() -> f32 {
    println!("What is the temperature you'd like to convert?");

    loop {
        let mut temperature = String::new();

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please enter a decimal like '32.1'");
                continue;
            },
        };

        return temperature;
    }
}

fn get_new_scale(temperature_scale: char) -> char {
    if temperature_scale == 'F' {
        'C'
    } else {
        'F'
    }
}

fn f_to_c(temp: f32) -> f32 {
    (temp + 32.0) / 1.8
}

fn c_to_f(temp: f32) -> f32 {
    (temp * 1.8) + 32.0
}

fn print_banner() {
    println!("|*****************************************************************|");
    println!("| Convert temperatures from Celsius to Fahrenheit (or vice-versa) |");
    println!("|*****************************************************************|");
}
