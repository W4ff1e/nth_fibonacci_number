use std::io;

fn main() {
    println!("I don't know math so if you want to be *technically* correct, please use 0 and 1 as your seed values.");
    loop {
        let first_seed = get_seed("first");

        let first_seed: usize = match first_seed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let second_seed = get_seed("second");

        let second_seed: usize = match second_seed.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Please enter the nth Fibonacci number to generate: ");
        let mut nth: String = String::new();

        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line!");

        let nth: usize = match nth.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
