use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

	let secret_number = rand::rng().random_range(100..=999);

	println!("The secret number is: {}", secret_number);

    println!("Guess the number:");
	println!("Input your first guess.");

	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Failed to read line");
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {println!("Please input a number."); continue;},
		};

		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {println!("You got it!");break;},
		}
	}
}
