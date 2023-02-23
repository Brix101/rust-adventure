use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let limit: u32 = 100;
        let secret_number = rand::thread_rng().gen_range(1..=limit);
        let hint_number = rand::thread_rng().gen_range(1..=10);
        let nearest_number = (secret_number / 10) * hint_number;
        let mut guessed_time: u32 = 10;

        println!("Guess the number!");
        println!("Hint!: {}, {}", nearest_number, hint_number);

        loop {
            let mut guess = String::new();

            match guessed_time {
                0 => {
                    println!("Game Over!!!");
                    println!("Correct number: {}", secret_number);
                    println!("-------------------------------------------------------------");
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
                    if num > limit {
                        println!("Please type a number lower than 100");
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
                    println!("You win!");
                    for _i in 1..10 {
                        print!("xo")
                    }
                    break;
                }
            }
        }
    }
}
