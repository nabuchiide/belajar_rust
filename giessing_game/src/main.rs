use core::num;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secreet number is : {}", secret_number);
    loop {
        println!("please input your number");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!"),
            Ordering::Greater => println!("Too big!!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
