use rand::Rng;
use std::io; // library for input output // library fro random number genration
fn main() {
    let guess_num = rand::thread_rng().gen_range(1..=100);
    println!("Guess Game\n");
    println!("Hint: number can be from 1 to 100\n");
    println!("Enter your guess\n");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("failed to read input");
    let guess = guess_num.to_string();
    if num == guess {
        println!("You guessed correct\n");
    } else {
        println!("incorrect guess secret number is {guess}\n");
    }
}
