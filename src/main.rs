use std::io::{self, Write};
use std::cmp::Ordering;
use rand::Rng;
use ansi_term::Colour;

fn main() {
    println!("Guess the number!");
    
    //let _enabled = ansi_term::enable_ansi_support();
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut num_guess = 0;

    //println!("The secret number is: {}", secret_number);

    loop {        
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", Colour::Red.bold().paint("Too Small!"));
                num_guess += 1;
            },
            Ordering::Greater => {
                println!("{}", Colour::Red.bold().paint("Too Big!"));
                num_guess += 1;
            },
            Ordering::Equal => {
                let win_string = format!("{}{}{}", "You win after ", num_guess, " guess(es)!");
                println!("{}", Colour::Green.bold().paint(win_string));
                break;
            }
        }
    }
    println!();
    print!("Press any key to finish the program!");
    io::stdout().flush().unwrap();
    

    io::stdin().read_line(&mut String::new()).unwrap();
}
