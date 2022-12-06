use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;


pub struct Puzzle<'a, T> {
    start: T,
    run: & 'a dyn Fn(&str, T) -> T,
    show:  & 'a dyn Fn(T) -> String,
}



pub struct Day<'a, T,S> {
    name: & 'a str,
    puzzle1: Puzzle<'a, T>,
    puzzle2: Puzzle<'a, S>,
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
        let res1 = (self.puzzle1.show)(self.reduce_input_lines(self.puzzle1.run, self.puzzle1.start.clone()));
        println!("Result1 is: {:?}", res1);
        let res2 = (self.puzzle2.show)(self.reduce_input_lines(self.puzzle2.run, self.puzzle2.start.clone()));
        println!("Result1 is: {:?}", res2);
    }
}