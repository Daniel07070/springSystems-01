//Solution to Assignment 1: Temperature Converter

const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main() {
    // Mutable starting temperature in Fahrenheit
    let mut temp_f: f64 = 32.0;

    // Converts F to C
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.1}°F = {:.1}°C", temp_f, temp_c);

    // For loop that converts and prints the next 5 integers
    for _ in 0..5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{:.1}°F = {:.1}°C", temp_f, temp_c);
    }
}


//Solution to Assignment 2: Number Analyzer
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Array of 10 integers
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];

    // For loop to check what to print
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 { //fizzbuzz has the highest precedence
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            if is_even(num) { //even or odd has the lowest precedence
                println!("{}: Even", num);
            } else {
                println!("{}: Odd", num);
            }
        }
    }

    
    let mut index = 0;
    let mut sum = 0;
    //while loop for sum of all numbers in array
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("Sum of all numbers: {}", sum);

    
    let mut largest = numbers[0];
    //for loop to find the largest number in the array
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }

    println!("Largest number: {}", largest);
}


//Solution to Assignment 3: Guessing Game
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Mutable secret number variable
    let mut secret = 21;

    
    let mut guess_count = 0; //number of guesses
    let mut guess; //guess value

    loop {
        // Simulated user input
        guess = match guess_count {
            0 => 30,
            1 => 15,
            2 => 21,
            _ => 0,
        };

        guess_count += 1;

        println!("Guess #{}: {}", guess_count, guess);

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Correct!");
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }

    println!("It took {} guesses.", guess_count);
}


