use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        // call the stdin function imported from io
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");
        // & means that this is a pointer to "guess" previously defined above

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // parse turns the number into a string

        println!("You guessed: {}", guess); 
        // curly braces act as 'arguments'

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}
