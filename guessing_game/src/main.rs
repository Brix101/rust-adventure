use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guessed_time: u32 = 5;
    loop {
        let mut guess = String::new();

        match guessed_time {
            0 => {
                println!("Game Over!!!");
                break;
            }
            _ => {
                println!("Remaining guess {}", guessed_time);
                println!("Please input your guess: ");
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
            }
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                let limit: u32 = 100;

                if num > limit {
                    println!("Please type a number lower than 100");
                }

                guessed_time -= 1;
                num
            }
            Err(e) => {
                println!("{e}, Please type a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
