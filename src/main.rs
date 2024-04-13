use num_bigint::BigUint;
use std::io;
fn main() {
    println!("I don't know math so if you want to be *technically* correct, please use 0 and 1 as your seed values.");
    loop {
        let first_seed = get_seed("first");

        let first_seed: BigUint = match first_seed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let second_seed = get_seed("second");

        let second_seed: BigUint = match second_seed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please enter the nth Fibonacci number to generate: ");
        let mut nth: String = String::new();

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line!");

        let nth: i64 = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = calculate(first_seed, second_seed, nth);

        println!("The {nth} digit off your fibonacci sequence is: {result}")
        /*
        let mut sequence = vec![first_seed, second_seed];

        for digit in 1..nth + 1 {
            if digit == nth {
                println!(
                    "The {nth}th digit of your fibonacci sequence is: {}",
                    sequence[digit]
                );
                break;
            }
            sequence.push(sequence[digit - 1] + sequence[digit]);

            // Test Code: (aaron-bond.better-comments)
            //// println!("Debug: Iteration: {digit}, Result: {}", sequence[digit + 1]);
        }
         */
    }
}

fn get_seed(seed_number: &str) -> String {
    println!("Please enter the {seed_number} seed value: ");
    let mut seed: String = String::new();

    io::stdin()
        .read_line(&mut seed)
        .expect("Failed to read line!");
    seed
}

fn calculate(mut f0: BigUint, mut f1: BigUint, digits_to_calc: i64) -> BigUint {
    for _ in 0..digits_to_calc {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}
