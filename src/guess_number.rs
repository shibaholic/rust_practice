use rand::Rng;
use std::io;
use std::io::prelude::*;

enum state {
    GenerateRandom,
    Guess,
    Correct
}

pub fn guess_number() {
    let mut game_state:state = state::GenerateRandom;
    let mut random_number = 0;
    let mut guess = String::new();
    loop {
        match game_state {
            state::GenerateRandom => {
                random_number = rand::thread_rng().gen_range(1..100);
                println!("Random number generated!");
                game_state = state::Guess
            }

            state::Guess => {
                guess.clear();
                print!("Guess the number: ");
                io::stdout().flush(); // allows the print above to not need ln
                io::stdin().lock().read_line(&mut guess).expect("reading user console won't fail");
                
                let guess_trim = guess.trim_end();

                // how to loop through chars in the string
                // for (i, c) in guess.as_bytes().iter().enumerate() {
                //     println!("{}: {:02x}", i, c);
                // }
                
                // following has turbofish operator
                let parse_result:Result<i32, String> = guess_trim.parse::<i32>().map_err(|e| e.to_string());
                if parse_result.is_err() { 
                    println!("Input value is invalid! No alphabetical characters allowed!");
                } else {
                    let guess_num = parse_result.unwrap();
                    if guess_num > random_number {
                        // bigger
                        println!("Your guess is bigger than number!");
                    } else if guess_num < random_number {
                        // smaller
                        println!("Your guess is smaller than number!");
                    } else if guess_num == random_number {
                        // correct
                        game_state = state::Correct;
                    }
                }
            }

            state::Correct => {
                println!("Your guess is correct!");
                std::process::exit(0);
            }
        }
    }
    
    
    
    
}