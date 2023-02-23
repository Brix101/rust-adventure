use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let (limit, secret_number, hint_number, nearest_number) = secret_generator();
        let mut guessed_time: u32 = 5;

        println!("<><><><><><><><><><><><><><><><><><><><><><><><><><>");
        println!("Guess the number!");
        println!("Hint!: {}, {}", nearest_number, hint_number);

        loop {
            let mut guess = String::new();

            match guessed_time {
                0 => {
                    println!("Correct number: {}", secret_number);
                    println!("!!!!!!!!!!!!!!!!!!!");
                    println!("!!!! Game Over !!!!");
                    println!("!!!!!!!!!!!!!!!!!!!");
                    break;
                }
                _ => {
                    println!("");
                    println!("<<<<<<<<<<000--- Remaining guess {guessed_time} ---000>>>>>>>>>>");
                    println!("Please input your guess: ");
                    io::stdin()
                        .read_line(&mut guess)
                        .expect("Failed to read line");
                }
            }

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => {
                    if num > limit {
                        println!("Please type a number lower than {limit}");
                        continue;
                    } else if num <= 0 {
                        println!("Please type a number greater than 0");
                        continue;
                    } else {
                        guessed_time -= 1;
                        num
                    }
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
                    println!("===================");
                    println!("===== You win =====");
                    println!("===================");
                    break;
                }
            }
        }
    }
}

fn secret_generator() -> (u32, u32, u32, u32) {
    let limit: u32 = 100;
    let secret_number = rand::thread_rng().gen_range(1..=limit);
    let hint_number = rand::thread_rng().gen_range(1..=10);
    let nearest_number = (secret_number / 10) * hint_number;

    return (limit, secret_number, hint_number, nearest_number);
}

