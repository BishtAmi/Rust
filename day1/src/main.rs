use rand::Rng;
use std::io;
// modified guess game
fn main() {
    let secret = rand::thread_rng().gen_range(1..=1000);
    println!("Guess the number HINT[1,1000]\n");

    // loop
    loop {
        println!("enter you guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed while input");
        let num: i32 = guess.trim().parse().expect("Input is not int");
        if num == secret {
            println!("You guess correct\n");
            break;
        } else {
            println!("Incorrect guess\n");
        }
        println!("If you want to guess again enter 1 else any other number\n");
        let mut more = String::new();
        io::stdin().read_line(&mut more).expect("failed");
        let wantmore:i32=more.trim().parse().expect("Input failed");
        if wantmore == 1 {
            continue;
        } else {
            println!("Thank you\n");
            break;
        }
    }
}
