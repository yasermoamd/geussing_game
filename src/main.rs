use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please inter your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess);

        let guess: u32 = match guess.trim().parse() {
            // handle invalid input
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {} ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
