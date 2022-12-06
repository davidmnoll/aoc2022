use regex::Regex;

use crate::days;

type Day5Type1 = Vec<Vec<char>>;
type Day5Type2 = Day5Type1;

pub const DAY : days::Day<Day5Type1, Day5Type2> = days::Day {
    puzzle1: days::Puzzle {
        start: vec![],
        run: &run1,
        show: &|xs|{
            let mut disp = "".to_string();
            for x in xs {
                disp.push(*x.last().unwrap())
            }
            disp
        },
    },
    puzzle2: days::Puzzle {
        start: vec![],
        run: &run2,
        show: &|xs|{
            let mut disp = "".to_string();
            for x in xs {
                disp.push(*x.last().unwrap())
            }
            disp
        },

    },
    name: "day5",
    
};

fn run1(line: & str, acc: Day5Type1) -> Day5Type1 {
    if line.is_empty() {
        return acc;
    }
    let mut acc_copy = acc.clone();
    let crates_re1 = Regex::new(r"\[([A-Z])\]").unwrap();
    let crates_re2 = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for crate_capture in crates_re1.captures_iter(line){
        let capture = crate_capture.get(1).unwrap();
        let column = capture.start() / 4;
        if column >= acc_copy.len(){
            acc_copy.resize(column + 1, vec![])
        }
        let character = capture.as_str().chars().nth(0).unwrap();
        let mut new_vec = vec![character];
        new_vec.append(& mut acc_copy[column]);
        acc_copy[column] = new_vec;
        // println!("Acc {:?} {:?} {:?}", acc_copy, column, acc_copy.len());
        // println!("Crate Capture{:?}", crate_capture.get(1).unwrap().start() / 4);
    }
    for move_capture in crates_re2.captures_iter(line){
        let num = move_capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = move_capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = move_capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
        for _i in 0..num {
            let _crate_to_move = acc_copy[from - 1].pop().unwrap();
            acc_copy[to - 1].push(_crate_to_move);
        // println!("Acc {:?} {:?} {:?}", acc_copy, (num, from, to ), acc_copy.len());
        }
    }

    return acc_copy;
}

fn run2 (line: &str, acc: Day5Type2) -> Day5Type2 {
    if line.is_empty() {
        return acc;
    }
    let mut acc_copy = acc.clone();
    let crates_re1 = Regex::new(r"\[([A-Z])\]").unwrap();
    let crates_re2 = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();
    for crate_capture in crates_re1.captures_iter(line){
        let capture = crate_capture.get(1).unwrap();
        let column = capture.start() / 4;
        if column >= acc_copy.len(){
            acc_copy.resize(column + 1, vec![])
        }
        let character = capture.as_str().chars().nth(0).unwrap();
        let mut new_vec = vec![character];
        new_vec.append(& mut acc_copy[column]);
        acc_copy[column] = new_vec;
        // println!("Acc {:?} {:?} {:?}", acc_copy, column, acc_copy.len());
        // println!("Crate Capture{:?}", crate_capture.get(1).unwrap().start() / 4);
    }
    for move_capture in crates_re2.captures_iter(line){
        let num = move_capture.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = move_capture.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = move_capture.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let final_size = acc_copy[from - 1].len() - num;
        let mut last_n = acc_copy[from - 1].split_off(final_size);
        // let _crate_to_move = acc_copy[from - 1].pop().unwrap();
        // println!("Acc {:?} {:?} {:?}", acc_copy, (num, from, to ), acc_copy.len());
        acc_copy[to - 1].append(& mut last_n);
    }

    return acc_copy;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1 () {
        let res = run1("    [D]     ", DAY.puzzle1.start);
        let res = run1("[N] [C]     ", res);
        let res = run1("[Z] [M] [P] ", res);
        let res = run1(" 1   2   3  ", res);
        let res = run1("move 1 from 2 to 1", res);
        let res = run1("move 3 from 1 to 3", res);
        let res = run1("move 2 from 2 to 1", res);
        let res = run1("move 1 from 1 to 2", res);
        let res = run1("", res);
        println!("result is {:?}", res);
        assert!(res[1][0] == 'M', "{:?} not equal 2", res);
    }

    #[test]
    fn test_run2 () {
        let res = run2("    [D]     ", DAY.puzzle2.start);
        let res = run2("[N] [C]     ", res);
        let res = run2("[Z] [M] [P] ", res);
        let res = run2(" 1   2   3  ", res);
        let res = run2("move 1 from 2 to 1", res);
        let res = run2("move 3 from 1 to 3", res);
        let res = run2("move 2 from 2 to 1", res);
        let res = run2("move 1 from 1 to 2", res);
        let res = run2("", res);
        println!("result is {:?}", res);
        assert!(res[1][0] == 'C', "{:?} not equal 2", res);
    }
}