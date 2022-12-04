use crate::days;

fn priority_from_char(char: char) -> u32{
    let mut b = [0; 2];
    char.encode_utf8(&mut b);
    let i = u32::from_be_bytes([0,0,0,b[0]]);
    return if i > 96 && i < 123{
        i - 96
    }else if i > 64 && i < 91 {
        i - 38
    }else {
        0
    };
}


pub fn run1(line: &str, acc: u32) -> u32 {
    if line.len() < 2 {
        return acc
    }
    let split_index = line.len() / 2;
    let (front, back) = line.split_at(split_index);
    let mut table: [bool;53] = [false;53];
    for char in front.chars() {
        table[(priority_from_char(char) as usize)] = true;
    };
    let mut score = 0;
    for char in back.chars() {
        let val = priority_from_char(char);
        if table[(val as usize)] {
            score = val;
        }
    }
    acc + score
}

pub fn run2(line: &str, acc: Day3Type2) -> Day3Type2 {
    let line_string = line.to_string();
    if line.is_empty(){
        acc
    }else{
        if acc.0.0.is_empty() {
            ((line_string, acc.0.1), acc.1)
        }else if acc.0.1.is_empty() {
            ((acc.0.0, line_string), acc.1)
        }else {
            let mut table: [u8;53] = [0;53];
            for char in acc.0.0.chars() {
                if table[(priority_from_char(char) as usize)] == 0 {
                    table[(priority_from_char(char) as usize)] = 1;
                }
            };
            for char in acc.0.1.chars() {
                if table[(priority_from_char(char) as usize)] == 1 {
                    table[(priority_from_char(char) as usize)] = 2;
                }
            };
            let mut badge = 0;
            for char in line.chars() {
                if table[(priority_from_char(char) as usize)] == 2 {
                    badge = priority_from_char(char);
                }
            };
            ((String::new(),String::new()), acc.1 + badge)
        }
    }
}

type Day3Type1 = u32;
type Day3Type2 = ((String, String), u32);

pub const DAY : days::Day<Day3Type1, Day3Type2> = days::Day {
    start1: 0,
    start2: ((String::new(),String::new()), 0),
    run1: &run1,
    run2: &run2,
    name: "day3"
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_map_right() {
        let mut x = priority_from_char('a');
        assert!(priority_from_char('a') == 1);
        assert!(priority_from_char('b') == 2);
        assert!(priority_from_char('p') == 16);
        assert!(priority_from_char('c') == 3);
        println!(" priority is : {:?}", x);
        x = priority_from_char('A');
        println!(" priority is : {:?}", x);
        assert!(priority_from_char('A') == 27);
        assert!(priority_from_char('L') == 38);
        assert!(priority_from_char('P') == 42);
        assert!(priority_from_char('v') == 22);
        assert!(priority_from_char('s') == 19);
        assert!(priority_from_char('t') == 20);
        assert!(priority_from_char('z') == 26);
        assert!(priority_from_char('Z') == 52);
        assert!(priority_from_char(' ') == 0);
    }

    #[test]
    fn run1_runs() {
        let mut res = run1("vJrwpWtwJgWrhcsFMMfFFhFp", 0);
        let res = run1("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", res);
        let res = run1("\n", res);
        let res = run1("PmmdzqPrVvPwwTWBwg", res);
        let res = run1("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", res);
        let res = run1("ttgJtRGJQctTZtZT", res);
        let res = run1("CrZsJsPPZsGzwwsLwLmpwMDw", res);
        let res = run1("yyyaTTaT", res);
        let res = run1("   ", res);
        let res = run1("   ", res);
        println!(" score is : {:?}", res);
        assert!(res == 158);
    }


    #[test]
    fn run2_runs() {
        let mut res = run2("vJrwpWtwJgWrhcsFMMfFFhFp", (( String::new(), String::new()),0));
        println!(" score is : {:?}", res);
        let res = run2("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL", res);
        println!(" score is : {:?}", res);
        let res = run2("PmmdzqPrVvPwwTWBwg", res);
        println!(" score is : {:?}", res);
        let res = run2("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn", res);
        println!(" score is : {:?}", res);
        let res = run2("ttgJtRGJQctTZtZT", res);
        println!(" score is : {:?}", res);
        let res = run2("CrZsJsPPZsGzwwsLwLmpwMDw", res);
        println!(" score is : {:?}", res);
        let res = run2("", res);
        println!(" score is : {:?}", res);
        assert!(res == ((String::new(), String::new()),70));
        assert!(true);
    }
}