use std::io;

fn main() {

    println!("guess the number");

    println!("Please input your guess");
    let mut guess = String::new();

    io::stdin().read_line( &mut guess)
        .expect("failed to decode");

    println!("You guessed: {}", guess)

}
