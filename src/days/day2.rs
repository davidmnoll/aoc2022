use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub fn run1(line: &str, acc: i32) -> i32 {
    let re = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    let matches = re.captures(line);
    return if matches.is_some() {
        // panic!("{:?}", matches);
        let matchesVal = matches.unwrap();
        let myMove = matchesVal.get(1).unwrap().as_str();
        let theirMove = matchesVal.get(2).unwrap().as_str();
        let score =  match [myMove, theirMove] {
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
        let matchesVal = matches.unwrap();
        let myMove = matchesVal.get(1).unwrap().as_str();
        let theirMove = matchesVal.get(2).unwrap().as_str();
        let score =  match [myMove, theirMove] {
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
        let res0 = run1("35", (vec![12, 14], 20));
        let res1 = run1("", res0);
        let res2 = run1("3", res1);
        println!("res2.0 {:?}", res2.0);
        println!("res2.1 {:?}", res2.1);
        assert!(res2.1 == 61);
    }


    #[test]
    fn run2_runs() {
        let mut res = run2("35", (vec![12, 14], (20, 21, 23)));
        res = run2("", res);
        res = run2("31", res);
        res = run2("32", res);
        res = run2("33", res);
        res = run2("", res);
        res = run2("35", res);
        res = run2("36", res);
        res = run2("20", res);
        res = run2("", res);
        res = run2("14", res);
        res = run2("36", res);
        res = run2("20", res);
        res = run2("", res);
        res = run2("54", res);
        res = run2("56", res);
        res = run2("50", res);
        res = run2("", res);
        println!("res2.0 {:?}", res.0);
        println!("res2.1 {:?}", res.1);
        assert!(res.1.0 == 160 && res.1.1 == 96 && res.1.2 == 91);
    }
}