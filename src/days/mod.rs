use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;



pub struct Day<'a, T,S> {
    name: & 'a str,
    start1: T,
    start2: S,
    run1: & 'a dyn Fn(&str, T) -> T,
    run2: & 'a dyn Fn(&str, S) -> S,
    show1: & 'a dyn Fn(T) -> String,
    show2: & 'a dyn Fn(S) -> String,
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
        return last;
    }

    pub fn output_results(&self) {
        let res1 = (self.show1)(self.reduce_input_lines(self.run1, self.start1.clone()));
        println!("Result1 is: {:?}", res1);
        let res2 = (self.show2)(self.reduce_input_lines(self.run2, self.start2.clone()));
        println!("Result1 is: {:?}", res2);
    }
}