use std::io;
use rand::Rng;

fn main() {
    
    let secret_number = rand
    println!("Guess the number!");
    println!("Pls input your guess!!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("failed to read line");

    print!("your guess {0}",guess);
}
