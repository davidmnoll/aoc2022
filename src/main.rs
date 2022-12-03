mod days;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use days::day1;
use std::vec;

fn main() {
    if env::args().len() == 2 {
        let first_argument: String = env::args().nth(1).unwrap();
        println!("You chose to run: {}", first_argument);
        if first_argument == "day1" {
            let result = reduce_input_lines(&first_argument, &day1::run1, ( vec![], i32::MIN));
            if result.is_ok() {
                println!("Result1 is: {}", result.unwrap().1);
            }else{
                println!("Result1 not found");
            }
            let result2 = reduce_input_lines(&first_argument, &day1::run2, ( vec![], (i32::MIN, i32::MIN, i32::MIN)));
            if result2.is_ok() {
                let result2_val = result2.unwrap();
                let result2_sum = result2_val.1.0 + result2_val.1.1 + result2_val.1.2;
                println!("Result2 is: {:?} => {:?}", result2_val.1, result2_sum);
            }else{
                println!("Result2 not found");
            }

        }
    }else if env::args().len() < 2{
        println!("please enter the day you would like to run e.g. for day 1 enter day1");
    }else{
        println!("too many arguments");

    }
}

fn reduce_input_lines<T>(filename: &String, function: &dyn Fn(&str, T) -> T, start_value: T) -> Result<T, &'static str> {
    if let Ok(lines) = read_lines(format!("./inputs/{}.txt", filename)){
        let mut last = start_value;
        for line in lines {
            last = function(line.unwrap().as_str(), last)
        }
        last = function("", last);
        return Ok(last)
    }else{
        return Err("could not read file")       
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}