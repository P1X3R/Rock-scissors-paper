use std::io::stdin;

use rand::Rng;

const OPTIONS: [&str; 3] = ["rock", "scissors", "paper"];

fn get_input(msg: &str) -> String {
    let mut input = String::new();
    println!("\n{msg}");
    stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "rock" => print!(""),
        "scissors" => print!(""),
        "paper" => print!(""),
        "q" => std::process::exit(400),
        _ => println!("There's no a valid option"),
    }

    return input;
}

fn generate_random_option(input: &str) {
    let mut rng = rand::thread_rng();
    let option_number = rng.gen_range(0..3);
    let option = OPTIONS[option_number];

    println!("{option}");

    // Handle options
    if option == input {
        println!("Tie!!");
    } else if input == "rock" && option == "scissors" {
        println!("You win!!");
    } else if input == "rock" && option == "paper" {
        println!("You lose");
    } else if input == "scissors" && option == "paper" {
        println!("You win!!");
    } else if input == "scissors" && option == "rock" {
        println!("You lose");
    } else if input == "paper" && option == "rock" {
        println!("You win!!");
    } else if input == "paper" && option == "scissors" {
        println!("You lose");
    }
}

fn main() {
    loop {
        let input = get_input("Select your option (rock, scissors or paper): ");
        generate_random_option(input.trim());
    }
}
