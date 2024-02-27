// Variables, Functions and control structures

fn main() {
    // main
    // practice exercises
    let factorial = calculate_factorial(25);
    println!("The factorial of 25 is: {}", factorial);

    let is_prime = is_prime(17);
    println!("Is 17 prime?: {}", is_prime);
}

fn immutableVariables(){
    // Variables are immutable by default
    let x = 1;
    println!("The value of x is: {}", x);
    // x = 2; // error
}

fn mutableVariables(){
    // If we want a variable to be mutable, we have to use "mut"
    let mut x = 1;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);
}

// Constant
const CONSTANT: i8 = 1;

fn shadowing(){
    let x = 1;
    let x = x + 23;
    let x = x - 23 / 23;
    println!("The value of x is: {}", x)
}

// Data types in Rust
fn scalars(){
    // Signed Integers
    let eight_bits: i8 = 1;
    let sixteen_bits: i16 = 1;
    let thirtytwo_bits: i32 = 1;
    let sixtyfour_bits: i64 = 1;
    let ahundredtwentyeight_bits: i128 = 1;
    // Unsigned Integers
    let eight_bits: u8 = 1;
    let sixteen_bits: u16 = 1;
    let thirtytwo_bits: u32 = 1;
    let sixtyfour_bits: u64 = 1;
    let ahundredtwentyeight_bits: u128 = 1;

    // Floats
    let thirtytwo_float: f32 = 2.65;
    let sixtyfour_float: f64 = 25.0;

    // Bool
    let boolean: bool = true;

    // Character
    let character:char = 'a';
}

fn compounds(){
    // Tuples
    let tup: (i32, f64, char) = (25, 8.36, 'r');

    // Arrays
    let arr: [i32; 4] = [1,2,3,4];
    let val1: i32 = arr[0];
    // let val2:i32 = arr[4]; // Error

    // For loop
    for i in arr.iter(){
        println!("{}", i)
    }

    // While
    let mut int = 0;
    while int<10{
        int = int + 1;
    }

    loop{
        int = int -10;
        break;
    }
}

fn my_function() -> i8{
    3
    // return 3;
}

//---------- Practice ----------//

fn calculate_factorial(number: u128) -> u128{
    if number == 0 || number == 1{
        return 1;
    }
    let mut result = number;
    for i in (1..number).rev(){
        result = result * i;
    }
    return result;
}

fn is_prime(number: u128) -> bool{
    let num: f64 = number as f64;
    if number > 1{
        for i in 2..(num.sqrt() as i128 + 1){
            if number as i128 % i == 0{
                return false;
            }
        }
    }
    true
}