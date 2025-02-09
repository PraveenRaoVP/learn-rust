use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let number = rand::rng()
        .random_range(1..=100);


    let mut no_of_tries: u32 = 5;

    loop {
        println!("Input your number: ");

        let mut guess = String::new();


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");


        let guess: u32 = guess.trim().parse().expect("Please Enter a Number!");

        match guess.cmp(&number) {
            Ordering::Less => {
                println!("Your guess is less than my number. Try again.");
                no_of_tries-=1;
            },
            Ordering::Greater => {
                println!("Your guess is more than my number. Try Again.");
                no_of_tries-=1;
            },
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }

        println!("No of Tries left: {}", no_of_tries);

        if no_of_tries == 0 {
            println!("The number of tries has ended. The number is: {}", number);
            break;
        }

    }

    println!("End Game.")

}
