use std::cmp::Ordering;
use std::io;
use rand::Rng;
// I/O library

fn main() { //entry point to our program
    // print string
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");


    //loop keyword create an infinite loop, well add a loop to give users more chances at guessing the number.
    loop {
        println!("Please input your guess");
        //create a variable to store a string
        let mut guess = String::new();

        io::stdin()
            // the read_line full job is to take whatever user types into standard input and append that into string(without overwriting its contents), the string arg need to be mutable so the method can change the string content

            //the & indicates that is argument is a reference, which gives you a way to let multiple parts of your  code access one piece of data without needing to copy that data into memory multiple times
            // REFERENCE and Variable are immutable by default
            .read_line(&mut guess)
            // expect is value if read_line return Result is Err. expect will be called, if the Result is Ok, it will hold to that value, so we can use it in the future
            .expect("Failed to read line");

        let guess: u32 = match guess.
            // trim will eliminate a white space at the beginning and end
            trim()
            // parse will parse the string into what type variable is, now the var type is i32, it will convert it to i32 if the type is other, it will convert if they can, parse also return result, OK and Err same as read_line
            .parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("You guessed: {}", guess);

        //cmp method compares 2 values and can be called on anything that can compared it takes a reference to whatever you want to compare with
        match guess.cmp(&secret_number) {
            //ordering  type is another enum and has variant Less, Greater and Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
