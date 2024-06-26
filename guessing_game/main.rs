extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guessing Game");

    let secretNumber = rand::thread_rng().gen_range(1..100);
    println!("{}", secretNumber);

    loop {
        println!("Please input your number");
        let mut guess: String = String::new();

        io::stdin() Stdin
            .read_line(buf: &mut guess) Result<usize, Error>
            .expect(msg:"failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num: u32) => num,
            Err(_) = continue,
        };

        println!("Your guessed number is: - {}", guess);

        match guess.cmp(&secretNumber) {
            Ordering::Less => println!("Too Small Number: "),
            Ordering::Greater => println!("Too Greater Number: "),
            Ordering::Equal => {println!("Great, You Won"); break;}
        }
    }
}
