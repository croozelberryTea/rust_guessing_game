use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        game_loop();

        println!("Would you like to play again? y/n");
        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line.");

        match play_again.trim() {
            play_again if play_again.to_lowercase() == "y" => continue,
            play_again if play_again.to_lowercase() == "n" => break,
            _ => {
                println!("invalid? just leave bro...");
                break;
            }
        }
    }
}

fn game_loop() {
    println!("Guess the number between 0 and 10!");
    let actual = random_number();
    let mut guess_count : u32 = 0;

    loop {
        println!("Input a number:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue
            }
        };
        guess_count += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&actual) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            }
        }
    }
    println!("You won in {} guesses! Goodbye!", guess_count);
}

fn random_number() -> u32 {
    return rand::thread_rng().gen_range(1..11);
}