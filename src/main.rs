mod days;

use std::env;

use days::day1;
use days::day2;
use days::day3;

fn main() {
    if env::args().len() == 2 {
        let first_argument = env::args().nth(1).unwrap();
        println!("You chose to run: {}", first_argument);
        match first_argument.as_str() {
            "day1" => day1::DAY.output_results(),
            "day2" => day2::DAY.output_results(),
            "day3" => day3::DAY.output_results(),
            _ => println!("FAILED")
        }
    }else if env::args().len() < 2{
        println!("please enter the day you would like to run e.g. for day 1 enter day1");
    }else{
        println!("too many arguments");

    }
}

