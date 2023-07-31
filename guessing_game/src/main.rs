use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("I'm going to think of a number between 1 and 100!");
    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop {
        println!("Guess? ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please guess a number!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
