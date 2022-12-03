use std::fs::File;
use std::io::{self, BufRead, Error};
use std::path::Path;

pub fn run1(line: &str, acc: (Vec<i32>, i32)) -> (Vec<i32>, i32){
    return if line.is_empty(){
        let my_sum = acc.0.into_iter().sum();
        if  my_sum > acc.1 {
            (vec![], my_sum)
        } else {
            (vec![], acc.1)
        }
    }else{
        ([acc.0, vec![line.parse::<i32>().unwrap()]].concat(), acc.1)
    }
}
pub fn run2(line: &str, acc: (Vec<i32>, (i32, i32, i32))) -> (Vec<i32>, (i32, i32, i32)){
    return if line.is_empty(){
        let my_sum = acc.0.into_iter().sum();
        if  my_sum > acc.1.0 {
            (vec![], (my_sum, acc.1.0, acc.1.1))
        } else if my_sum > acc.1.1 {
            (vec![], (acc.1.0, my_sum, acc.1.1))
        } else if my_sum > acc.1.2 {
            (vec![], (acc.1.0, acc.1.1, my_sum))
        } else{
            (vec![], acc.1)
        }
    }else{
        ([acc.0, vec![line.parse::<i32>().unwrap()]].concat(), acc.1)
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

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
    // let file = File::open(filename)?;
    // Ok(io::BufReader::new(file).lines())
// }


pub fn output_results() {
    let day = "day1".to_string();
    let result = reduce_input_lines(&day, &run1, ( vec![], i32::MIN));
    if result.is_ok() {
        println!("Result1 is: {}", result.unwrap().1);
    }else{
        println!("Result1 not found");
    }
    let result2 = reduce_input_lines(&day, &run2, ( vec![], (i32::MIN, i32::MIN, i32::MIN)));
    if result2.is_ok() {
        let result2_val = result2.unwrap();
        let result2_sum = result2_val.1.0 + result2_val.1.1 + result2_val.1.2;
        println!("Result2 is: {:?} => {:?}", result2_val.1, result2_sum);
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