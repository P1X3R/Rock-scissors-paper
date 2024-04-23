use std::io::stdin;

const OPTIONS: (&str, &str, &str) = ("rock", "scissors", "paper");

fn get_input(msg: &str) {
    let mut input = String::new();
    println!("{msg}");
    stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "rock" => print!(""),
        "scissors" => print!(""),
        "paper" => print!(""),
        "q" => std::process::exit(400),
        _ => println!("There's no a valid option")
    }
}

fn main() {
    loop {
        get_input("Select your option (rock, scissors or paper): ");
    }
}
