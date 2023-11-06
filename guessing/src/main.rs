use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number");
    
    let secret = rand::thread_rng().gen_range(1..=100);
    // println!("{secret}");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
    
        let guess: u32 = guess.trim()
            .parse()
            .expect("Please type a number!");
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Your guess is greater than the correct answer."),
            Ordering::Less => println!("Your guess is less than the correct answer."),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        };        
    }
}
