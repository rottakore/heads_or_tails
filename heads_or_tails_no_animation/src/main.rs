// Dependencies >>
use rand::Rng;
use std::io::{stdin, stdout, Write, Read};
// << Dependencies

// heads_or_tails function >>
fn main() {

	//Choosing random number >>
	let coin_side = rand::thread_rng()
		.gen_range(0..2);
	// << Choosing random number

	// Main text >>
	print!("Pick a side [Heads/Tails]: ");
	// << Main text
	
	// To force user input in the same line as print!
	let _ = stdout()
		.flush();
	// << To force user input in the same line as print!
	
	// Asking user to choose a option >>
	let mut user_answer = String::new();
	stdin()
		.read_line(&mut user_answer)
		.unwrap();
	// << Asking user to choose a option
	
	// Formatting user input to uppercase >>
	*&mut user_answer = user_answer
		.to_uppercase();
	// << Formatting user input to uppercase

	// Converting "HEADS" to "0" if user selected HEADS >>
	if user_answer
		.trim() == "HEADS" {
			*&mut user_answer = String::from("0");
		} 
	// << Converting "HEADS" to "0" if user selected HEADS
	
	// Converting "TAILS" to "1" if user selected TAILS >>
	else if user_answer
		.trim() == "TAILS" {
			*&mut user_answer = String::from("1");
		} 
	// << Converting "TAILS" to "1" if user selected TAILS
	
	// Ending game if user input doesn't match a coin side >>
	else {
		println!("You answered wrong! End of the game.");
		
		// Pausing the program until user hits enter >>
		stdin()
			.read(&mut [0])
			.unwrap();
		// << Pausing the program until user hits enter
		
	}
	// << Ending game if user input doesn't match a coin side
	
	// Copying user_answer to a integer variable called "user_answer_code" >>
	let user_answer_code: u32 = user_answer
		.trim()
		.parse()
		.unwrap();
	// << Copying user_answer to a integer variable called "user_answer_code"
	
	// Showing a winning result if user_answer_code matches coin_side >>
	if user_answer_code == coin_side {
		println!("You guessed right, Congratulations!");
		
		// Pausing the program until user hits enter >>
		stdin()
			.read(&mut [0])
			.unwrap();
		// << Pausing the program until user hits enter
	}
	// << Showing a winning result if user_answer_code matches coin_side
	
	// Showing a losing result if user_answer_code doesn't match coin_side >>
	else {
		println!("Oops, you didn't won this time!");
		
		// Pausing the program until user hits enter >>
		stdin()
			.read(&mut [0])
			.unwrap();
		// << Pausing the program until user hits enter
	}
	// << Showing a losing result if user_answer_code doesn't match coin_side

}
// << heads_or_tails function
