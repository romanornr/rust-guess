use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 10);
    println!("guess the number");

    loop {
        println!("Please input your guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("failed to decode");

        let guess: u32 = guess.trim().parse()
            .expect("Please type in a number");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => println!("You win")
        }

        println!("You guessed: {}", guess)
    }

}
