use rand::Rng;
use std::io; // library for input output // library fro random number genration
fn main() {
    let screat_num = rand::thread_rng().gen_range(1..=100);
    println!("Guess Game\n");
    println!("Hint: number can be from 1 to 100\n");
    println!("Enter your guess\n");
    let mut num = String::new();
    io::stdlin()
        .read_line(&mut num)
        .expect("failed to read input");
    if (num == screat_num) {
        println("You guessed correct\n");
    } else {
        println("incorrect guess\n");
    }
}
