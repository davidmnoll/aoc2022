use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;
use regex::Regex;

pub fn run1(line: &str, acc: i32) -> i32 {
    let re = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    let matches = re.captures(line);
    return if matches.is_some() {
        // panic!("{:?}", matches);
        let matches_val = matches.unwrap();
        let my_move = matches_val.get(1).unwrap().as_str();
        let their_move = matches_val.get(2).unwrap().as_str();
        let score =  match [my_move, their_move] {
            ["A","X"] => 4,
            ["A","Y"] => 8,
            ["A","Z"] => 3,
            ["B","X"] => 1,
            ["B","Y"] => 5,
            ["B","Z"] => 9,
            ["C","X"] => 7,
            ["C","Y"] => 2,
            ["C","Z"] => 6,
            _ => 0
        };
        acc + score
    } else {
        acc
    }
}

pub fn run2(line: &str, acc: i32) -> i32 {
    let re = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    let matches = re.captures(line);
    return if matches.is_some() {
        // panic!("{:?}", matches);
        let matches_val = matches.unwrap();
        let my_move = matches_val.get(1).unwrap().as_str();
        let their_move = matches_val.get(2).unwrap().as_str();
        let score =  match [my_move, their_move] {
            ["A","X"] => 3,
            ["A","Y"] => 4,
            ["A","Z"] => 8,
            ["B","X"] => 1,
            ["B","Y"] => 5,
            ["B","Z"] => 9,
            ["C","X"] => 2,
            ["C","Y"] => 6,
            ["C","Z"] => 7,
            _ => 0
        };
        acc + score
    } else {
        acc
    }
}

fn reduce_input_lines<T>(filename: &String, function: &dyn Fn(&str, T) -> T, start_value: T) -> Result<T, Error> 
    where String: AsRef<Path>{
    let file_path = format!("./inputs/{}.txt", filename);
    let file = File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut last = start_value;
    for line in lines {
        last = function(line.unwrap().as_str(), last)
    }
    last = function("", last);
    return Ok(last)
}


pub fn output_results() {
    let day = "day2".to_string();
    let result = reduce_input_lines(&day, &run1, 0);
    if result.is_ok() {
        println!("Result1 is: {}", result.unwrap());
    }else{
        println!("Result1 not found");
    }
    let result2 = reduce_input_lines(&day, &run2, 0);
    if result2.is_ok() {
        let result2_val = result2.unwrap();
        println!("Result2 is: {:?} ", result2_val);
    }else{
        println!("Result2 not found");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1_runs() {
        let res = run1("A X", 0);
        let res = run1("B Y", res);
        let res = run1("C Z", res);
        let res = run1("C Y", res);
        println!("res2 {:?}", res);
        assert!(res == 17);
    }


    #[test]
    fn run2_runs() {
        let res = run2("A X",0);
        let res = run2("B Y", res);
        let res = run2("C Z", res);
        let res = run2("C Y", res);
        println!("res2 {:?}", res);
        assert!(res == 21);
    }
}