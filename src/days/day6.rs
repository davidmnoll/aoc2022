use std::collections::HashSet;

use crate::days;

type Day6Type1 = u32;
type Day6Type2 = u32;

pub const DAY : days::Day<Day6Type1, Day6Type2> = days::Day {
    puzzle1: days::Puzzle {
        start: 0,
        run: &run1,
        show: &|x|{format!("{:?}", x)},
    },
    puzzle2: days::Puzzle {
        start:0, 
        run: &run2,
        show: &|x|{format!("{:?}", x)},
    },

    name: "day6",
};


fn run1(line: & str, acc: Day6Type1) -> Day6Type1 {
    if line.is_empty() {
        return acc;
    }
    let mut c = 0u32;
    let mut queue = std::collections::VecDeque::<char>::with_capacity(3);
    for char in line.chars() {
        c += 1;
        let set = queue.iter().cloned().collect::<HashSet<char>>();
        // println!("{:?} : {:?} : {:?}", queue.clone(), set, char );
        if set.len() == 3 && !set.contains(&char){
            break;
        }else{
            queue.push_back(char);
            if c > 3 {
                queue.pop_front();
            }
        }
    }
    return c;
}

fn run2 (line: &str, acc: Day6Type2) -> Day6Type2 {
    if line.is_empty() {
        return acc;
    }
    let mut c = 0u32;
    let mut queue = std::collections::VecDeque::<char>::with_capacity(13);
    for char in line.chars() {
        c += 1;
        let set = queue.iter().cloned().collect::<HashSet<char>>();
        // println!("{:?} : {:?} : {:?}", queue.clone(), set, char );
        if set.len() == 13 && !set.contains(&char){
            break;
        }else{
            queue.push_back(char);
            if c > 13 {
                queue.pop_front();
            }
        }
    }
    return c;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1 () {
        let res = run1("bvwbjplbgvbhsrlpgdmjqwftvncz", DAY.puzzle1.start);
        assert!(res == 5, "{:?} not equal 5", res);
        let res = run1("nppdvjthqldpwncqszvftbrmjlhg", DAY.puzzle1.start);
        assert!(res == 6, "{:?} not equal 6", res);
        let res = run1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", DAY.puzzle1.start);
        assert!(res == 10, "{:?} not equal 10", res);
        let res = run1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", DAY.puzzle1.start);
        assert!(res == 11, "{:?} not equal 11", res);
    }

    #[test]
    fn test_run2 () {
        let res = run2("mjqjpqmgbljsphdztnvjfqwrcgsmlb", DAY.puzzle2.start);
        assert!(res == 19, "{:?} not equal 19", res);
        let res = run2("bvwbjplbgvbhsrlpgdmjqwftvncz", DAY.puzzle2.start);
        assert!(res == 23, "{:?} not equal 23", res);
        let res = run2("nppdvjthqldpwncqszvftbrmjlhg", DAY.puzzle2.start);
        assert!(res == 23, "{:?} not equal 23", res);
        let res = run2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", DAY.puzzle2.start);
        assert!(res == 29, "{:?} not equal 29", res);
        let res = run2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", DAY.puzzle2.start);
        assert!(res == 26, "{:?} not equal 26", res);
    }
}