use crate::days;

pub fn priority_from_char(char: char) -> u32{
    let mut b = [0; 2];
    char.encode_utf8(&mut b);
    let i = u32::from_be_bytes([0,0,0,b[0]]);
    return if i > 96 && i < 123{
        i - 97
    }else if i > 64 && i < 91 {
        i - 39
    }else {
        u32::MAX
    };
}


pub fn run1(line: &str, acc: u32) -> u32 {
    if line.len() < 2 {
        return acc
    }
    let split_index = line.len() / 2;
    let (front, back) = line.split_at(split_index);
    let mut table: [bool;52] = [false;52];
    for char in front.chars() {
        let index = priority_from_char(char) as usize;
        if index < 52 {
            table[(index)] = true;
        }
    };
    let mut score = 0;
    for char in back.chars() {
        let val = priority_from_char(char);
        if val < 52 {
            if table[(val as usize)] {
                score = val + 1;
            }
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
            let mut table: [u8;52] = [0;52];
            for char in acc.0.0.chars() {
                let index = priority_from_char(char) as usize;
                if index < 52 {
                    if table[(index)] == 0 {
                        table[(index)] = 1;
                    }
                }
            };
            for char in acc.0.1.chars() {
                let index = priority_from_char(char) as usize;
                if index < 52 {
                    if table[(index)] == 1 {
                        table[(index)] = 2;
                    }
                }
            };
            let mut badge = 0;
            for char in line.chars() {
                let index = priority_from_char(char) as usize;
                if index < 52 {
                    if table[index ] == 2 {
                        badge = (index + 1) as u32;
                    }
                }
            };
            ((String::new(),String::new()), acc.1 + badge)
        }
    }
}

type Day3Type1 = u32;
type Day3Type2 = ((String, String), u32);

pub const DAY : days::Day<Day3Type1, Day3Type2> = days::Day {
    puzzle1: days::Puzzle {
        start: 0,
        run: &run1,
        show: &|x|{format!("{:?}", x)},
    },
    puzzle2: days::Puzzle {
        start: ((String::new(),String::new()), 0),
        run: &run2,
        show: &|x|{format!("{:?}", x.1)},
    },
    name: "day3",
};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_map_right() {
        let mut x = priority_from_char('a');
        assert!(priority_from_char('a') == 0);
        assert!(priority_from_char('b') == 1);
        assert!(priority_from_char('p') == 15);
        assert!(priority_from_char('c') == 2);
        println!(" priority is : {:?}", x);
        x = priority_from_char('A');
        println!(" priority is : {:?}", x);
        assert!(priority_from_char('A') == 26);
        assert!(priority_from_char('L') == 37);
        assert!(priority_from_char('P') == 41);
        assert!(priority_from_char('v') == 21);
        assert!(priority_from_char('s') == 18);
        assert!(priority_from_char('t') == 19);
        assert!(priority_from_char('z') == 25);
        assert!(priority_from_char('Z') == 51);
        assert!(priority_from_char(' ') == u32::MAX);
    }

    #[test]
    fn run1_runs() {
        let res = run1("vJrwpWtwJgWrhcsFMMfFFhFp", DAY.puzzle1.start);
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
        let res = run2("vJrwpWtwJgWrhcsFMMfFFhFp", DAY.puzzle2.start);
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