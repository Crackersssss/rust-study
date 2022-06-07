use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let rand_number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();

        io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                println!("Please re-enter the number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&rand_number) {
            Ordering::Less => println!("TOO small!"),
            Ordering::Greater => println!("TOO big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
         
    }
}
