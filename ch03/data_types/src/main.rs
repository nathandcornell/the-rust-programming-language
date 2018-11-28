fn main() {
    // Compiler error! "cannot infer type"
    // let unknowable = "123".parse().expect("Not a number");
    // println!("The value of unknownable is: {}", unknownable)

    let known:u32 = "123".parse().expect("Not a number");
    println!("The value of known is: {}", known);

    // Integer Types
    let unsigned_8_bit:u8 = std::u8::MAX;
    let unsigned_16_bit:u16 = std::u16::MAX;
    let unsigned_32_bit:u32 = std::u32::MAX;
    let unsigned_64_bit:u64 = std::u64::MAX;
    let unsigned_128_bit:u128 = std::u128::MAX;
    let unsigned_arch:usize = std::usize::MAX;

    let signed_8_bit:i8 = std::i8::MIN;
    let signed_16_bit:i16 = std::i16::MIN;
    let signed_32_bit:i32 = std::i32::MIN;
    let signed_64_bit:i64 = std::i64::MIN;
    let signed_128_bit:i128 = std::i128::MIN;
    let signed_arch:isize = std::isize::MIN;

    println!("The value of unsigned_8_bit is: {}", unsigned_8_bit);
    println!("The value of unsigned_16_bit is: {}", unsigned_16_bit);
    println!("The value of unsigned_32_bit is: {}", unsigned_32_bit);
    println!("The value of unsigned_64_bit is: {}", unsigned_64_bit);
    println!("The value of unsigned_128_bit is: {}", unsigned_128_bit);
    println!("The value of unsigned_arch is: {}", unsigned_arch);
    println!("The value of signed_8_bit is: {}", signed_8_bit);
    println!("The value of signed_16_bit is: {}", signed_16_bit);
    println!("The value of signed_32_bit is: {}", signed_32_bit);
    println!("The value of signed_64_bit is: {}", signed_64_bit);
    println!("The value of signed_128_bit is: {}", signed_128_bit);
    println!("The value of signed_arch is: {}", signed_arch);
    
    // Integer Literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // (u8 only)

    println!("The value of decimal is: {}", decimal);
    println!("The value of hex is: {}", hex);
    println!("The value of octal is: {}", octal);
    println!("The value of binary is: {}", binary);
    println!("The value of byte is: {}", byte);

    // Integer Overflow (2's compliment wrapping)
    let one_over:u8 = 256; // wraps to '0'
    let two_over:u8 = 257; // wraps to '1'

    println!("The value of one_over is: {}", one_over);
    println!("The value of two_over is: {}", two_over);

    // Floating-Point Types
    let sixty_four:f64 = std::f64::MAX;
    let thirty_two:f32 = std::f32::MAX;
    let default_float_type = 3.14159; // f64

    println!("The value of sixty_four is: {}", sixty_four);
    println!("The value of thirty_two is: {}", thirty_two);
    println!("The value of default_float_type is: {}", default_float_type);

    // Mathmatical Operators
    let sum = 5 + 7;
    let difference = 10.0 - 0.2;
    let product = 3 * 4;
    let quotient = 45 / 7;
    let remainder = 21 % 5;

    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient);
    println!("The value of remainder is: {}", remainder);

    // Booleans
    let false_value = false;
    let true_value:bool = true;

    println!("The value of false_value is: {}", false_value);
    println!("The value of true_value is: {}", true_value);

    // Character Type
    let z = 'z';
    let zed = 'ℤ';
    let poop:char = '💩';

    println!("The value of z is: {}", z);
    println!("The value of zed is: {}", zed);
    println!("The value of poop is: {}", poop);

    /******************
     * Compound Types *
     ******************/
    // Tuples
    let tuple = ("Foo", 4, 3.14159);
    let tuple_typed:(char, i32, f64) = ('🍻', 4, 3.14159);

    println!("The value of tuple.0 is: {}", tuple.0);
    println!("The value of tuple.1 is: {}", tuple.1);
    println!("The value of tuple.2 is: {}", tuple.2);

    println!("The value of tuple_typed.0 is: {}", tuple_typed.0);
    println!("The value of tuple_typed.1 is: {}", tuple_typed.1);
    println!("The value of tuple_typed.2 is: {}", tuple_typed.2);

    // Arrays
    let array = [1, 2, 3, 4, 5];
    let array_typed:[i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of array[3] is: {}", array[3]);
    // Compiler error! "index out of bounds: the len is 5 but the index is 5"
    // println!("The value of array[5] is: {}", array[5]);
}
