use std::io;
use rand::Rng;
use colored::*;
use std::cmp::Ordering;

pub fn paly(){
    println!("Guess!");
    let secret_num = rand::thread_rng().gen_range(0,101);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("faild to read line");

        let guess: u32 = match guess.trim().parse() {
         Ok(num) => num,
         Err(_) => {
            println!("type num");
            continue;   
         }
        };
        
        println!("you guess {}", guess );    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}", "small".red()),
            Ordering::Greater => println!("{}", "big".red()),
            Ordering::Equal => {
                println!("{}", "correct".green());
                break;
            },
        }
    }
}