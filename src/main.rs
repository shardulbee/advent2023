use advent2023::solutions;
use std::io::{self, Write};

fn main() {
    let mut day_input = String::new();
    let mut test_input = String::new();

    println!("Which day do you want to run? ");
    print!("Input: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut day_input).unwrap();
    let day_input = day_input.trim();

    println!("Testmode? Enter Y/N.");
    print!("Input: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut test_input).unwrap();
    let test_input = test_input.trim();

    let test_mode = match test_input {
        "Y" => true,
        "N" => false,
        _ => panic!("Invalid input!. Only Y/N are supported."),
    };

    match day_input {
        "1" => solutions::DayOne::run(test_mode),
        "2" => solutions::DayTwo::run(test_mode),
        "3" => solutions::DayThree::run(test_mode),
        "4" => solutions::DayFour::run(test_mode),
        _ => println!("Not implemented yet"),
    }
}
