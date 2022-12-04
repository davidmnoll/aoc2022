use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;



pub struct Day<'a, T,S> {
    name: & 'a str,
    run1: & 'a dyn Fn(&str, T) -> T,
    run2: & 'a dyn Fn(&str, S) -> S,
    start1: T,
    start2: S,
}

impl<'a,T,S> Day<'a, T,S> 
    where 
        T: std::fmt::Debug + Clone,
        S: std::fmt::Debug + Clone
    {
    fn reduce_input_lines<U: std::fmt::Debug>(&self, function: &dyn Fn(&str, U) -> U, start_value: U) -> U  
        where String: AsRef<Path>{
        let filename = self.name;
        let file_path = format!("./inputs/{}.txt", filename);
        let file = File::open(file_path).unwrap();
        let lines = io::BufReader::new(file).lines();
        let mut last = start_value;
        for line in lines {
            last = function(line.unwrap().as_str(), last)
        }
        last = function("", last);
        println!("Result1 is: {:?}", last);
        return last;
    }

    pub fn output_results(&self) {
        self.reduce_input_lines(self.run1, self.start1.clone());
        self.reduce_input_lines(self.run2, self.start2.clone());
    }
}