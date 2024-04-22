use std::cmp::Ordering;
use std::io;
fn main() {
    let riddle = String::from("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
    let correct_answer = String::from("The letter e");

    let mut tries = 1;

    loop {
        let mut user_answer = String::new();
        println!("{}", riddle);
        io::stdin()
            .read_line(&mut user_answer)
            .expect("type something");

        let user_answer = user_answer.trim().to_string();
        match user_answer.cmp(&correct_answer) {
            Ordering::Equal => {
                println!("Number of trials: {}", tries);
                break;
            }
            _ => {
                tries += 1;
                continue;
            }
        };
    }
}
