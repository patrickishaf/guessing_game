use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut number_of_tries = 1;

    println!("The secret number has been generated. MWAHAHAHAHA!. Please enter your guess");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Please enter a number");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => {
                number_of_tries += 1;
                println!("Too big");
            },
            Ordering::Less => {
                number_of_tries += 1;
                println!("Too small");
            },
            Ordering::Equal => {
                println!("After a total of {number_of_tries} tries, YOU WIN!");
                break;
            },
        }
    }
}