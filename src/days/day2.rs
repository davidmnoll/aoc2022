use regex::Regex;

use crate::days;

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

type Day2Type1 = i32;
type Day2Type2 = i32;

pub const DAY : days::Day<Day2Type1, Day2Type2> = days::Day {
    start1: 0,
    start2: 0,
    run1: &run1,
    run2: &run2,
    name: "day2"
};



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