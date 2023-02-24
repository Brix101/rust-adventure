use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let mut diff_input = String::new();

        println!("<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>");
        println!("<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>");
        println!("<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>");
        println!("Select Difficulty: [1]easy | [2]medium | [3]hard | [0]quit ");

        io::stdin()
            .read_line(&mut diff_input)
            .expect("Failed to read line");

        let mut diffculty: u32 = match diff_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}, Please type a number!");
                continue;
            }
        };

        match diffculty {
            1 => diffculty = 10,
            2 => diffculty = 5,
            3 => diffculty = 3,
            0 => break,
            _ => continue,
        }

        if diffculty > 0 {
            let GeneratedSecret { limit, secret_number, hint_number, nearest_number } = secret_generator();

            println!("<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>0<>");
            println!("Guess the number!");
            println!("Hint!: {}, {}", nearest_number, hint_number);

            loop {
                let mut guess = String::new();

                match diffculty {
                    0 => {
                        println!("Correct number: {}", secret_number);
                        println!("===================");
                        println!("==== Game Over ====");
                        println!("===================");
                        break;
                    }
                    _ => {
                        println!("");
                        println!("<<<<<<<<<<000--- Remaining guess {diffculty} ---000>>>>>>>>>>");
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
                            diffculty -= 1;
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
}

#[derive(Debug)]
struct GeneratedSecret {
    limit: u32,
    secret_number: u32,
    hint_number: u32,
    nearest_number: u32
}

fn secret_generator() -> GeneratedSecret {
    let limit: u32 = 100;
    let secret_number = rand::thread_rng().gen_range(1..=limit);
    let hint_number = rand::thread_rng().gen_range(1..=10);
    let nearest_number = (secret_number / 10) * hint_number;

    return GeneratedSecret { limit, secret_number, hint_number,  nearest_number }
}
