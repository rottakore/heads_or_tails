// Dependencies >>
use rand::Rng;
use std::io::{stdin, stdout, Read, Write};
use std::{thread, time};
use throbber::Throbber;
// << Dependencies

// Pause function >>
fn pause() {
	let mut stdout = stdout();
	stdout
	    .write(b"")
		    .unwrap();
	stdout
	    .flush()
		    .unwrap();
	stdin()
	    .read(&mut [0])
		    .unwrap();
}
// << Pause function

// HeadsOrTails function >>
fn main() {

	// Coin flipping animation >>
	let mut loadingAnimation = Throbber::new()
	    .message("The coin is flipping..."
		    .to_string())
		        .frames(&throbber::ROTATE_F);
	loadingAnimation
	    .start();
	thread::sleep(time::Duration::from_millis(2000));

	//Choosing random number >>
	let mut coinSide = rand::thread_rng()
	    .gen_range(0..2);
	// << Choosing random number

	loadingAnimation.success("Coin flipped!"
	    .to_string());
	loadingAnimation
	    .end();
	// << Coin flipping animation

	// Asking for user input and converting to uppercase >>
	print!("Pick a side [Heads/Tails]: ");
	stdout()
	    .flush();
	let mut userAnswer = String::new();
	stdin()
	    .read_line(&mut userAnswer)
		    .unwrap();
	*&mut userAnswer = userAnswer
	    .to_uppercase();
	// << Asking for user input and converting to uppercase

	// Converting "HEADS" to zero >>
	if userAnswer
	    .trim() == "HEADS" {
		*&mut userAnswer = String::from("0");
	// << Converting "HEADS" to zero
	
	// Converting "TAILS" to one >>
	} else if userAnswer
	    .trim() == "TAILS" {
		*&mut userAnswer = String::from("1");
	// << Converting "TAILS" to one
	
	// Ending game if user input doesn't match a coin side >>
	} else {
		println!("You answered wrong! End of the game.");
		pause();
	}
	// << Ending game if user input doesn't match a coin side
	
	// Copying userAnswer to a integer variable called "userAnswerCode" >>
	let userAnswerCode: u32 = userAnswer
		.trim()
			.parse()
				.unwrap();
	// << Copying userAnswer to a integer variable called "userAnswerCode"
	
	// Showing a winning result if userAnswerCode matches coinSide >>
	if userAnswerCode == coinSide {
		println!("You guessed right, Congratulations!");
		pause();
	// << Showing a winning result if userAnswerCode matches coinSide
	
	// Showing a losing result if userAnswerCode doesn't match coinSide >>
	} else {
		println!("Oops, you didn't won this time!");
		pause();
	}
	// << Showing a losing result if userAnswerCode doesn't match coinSide

}
// << HeadsOrTails function
