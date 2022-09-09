// guess the number https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// you have to run it locally as it require some input.

// for input scope.
use std::io;

// this is the main function
fn main() {
    // introducing our program with some text. 
    println!("Guess the number!");
    println!("Please input your guess");

    // creating a empty instance of String and making the variable mutable so that we can modify it later
    let mut guess = String::new();

    // reading line from console, if fail, we will show a message "Failed to read line"
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // showing our input string in console.
    println!("You guessed: {guess}");
}
