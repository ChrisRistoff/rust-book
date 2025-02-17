use rand::{self, Rng};

fn main() {
    println!("Guess the number");

    println!("Please input your guess");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You gussed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Enter a valid number");

    let random_number = rand::rng().random_range(1..=100);

    if guess == random_number {
        println!("You win");
    } else {
        println!("You lose");
    }

    println!("Random number was {}", random_number);
}
