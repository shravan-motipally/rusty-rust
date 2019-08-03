use std::io;

fn main() {
	println!("Guess the number!");
	println!("Please input your guess.");
	let mut guess = String::new();
	let failure_msg = String::new();

	io::stdin().read_line(&mut guess)
		.expect(failure_msg);

    println!("You guessed: {}", guess);

	println!("Guess another number!");
	io::stdin().read_line(&mut guess)
		.expect(failure_msg);
}
