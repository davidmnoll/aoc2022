use crate::days;
use crate::days::day3::priority_from_char;

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
    acc
}

fn run2 (line: &str, acc: Day6Type2) -> Day6Type2 {
    if line.is_empty() {
        return acc;
    }
    acc
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
        let res = run2("bvwbjplbgvbhsrlpgdmjqwftvncz", DAY.puzzle1.start);
        assert!(res == 5, "{:?} not equal 5", res);
        let res = run2("nppdvjthqldpwncqszvftbrmjlhg", DAY.puzzle1.start);
        assert!(res == 6, "{:?} not equal 6", res);
        let res = run2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", DAY.puzzle1.start);
        assert!(res == 10, "{:?} not equal 10", res);
        let res = run2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", DAY.puzzle1.start);
        assert!(res == 11, "{:?} not equal 11", res);
    }
}