mod days;

use std::env;

use days::day1;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
use days::day6;

fn main() {
    let day_match = |x: String|{
        match x.as_str() {
            "day1" => day1::DAY.output_results(),
            "day2" => day2::DAY.output_results(),
            "day3" => day3::DAY.output_results(),
            "day4" => day4::DAY.output_results(),
            "day5" => day5::DAY.output_results(),
            "day6" => day6::DAY.output_results(),
            _ => println!("FAILED")
        }
    };       
    let run_all = ||{
        let now = std::time::Instant::now();
        for i in 0..6 {
            day_match(format!("day{}", i + 1))
        };
        println!("Total time: {:?}s", now.elapsed().as_secs_f32())
    };
 
    if env::args().len() == 2 {
        let first_argument = env::args().nth(1).unwrap();
        println!("You chose to run: {}", first_argument.clone());
       if first_argument != "all" {
            day_match(first_argument.clone());
        }else{
            run_all();
        }
    }else if env::args().len() < 2{
        println!("You chose to run: all");
        run_all();
    }else{
        println!("too many arguments");

    }
}

