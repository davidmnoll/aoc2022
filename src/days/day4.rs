use regex::Regex;

use crate::days;

type Day4Type1 = u32;
type Day4Type2 = u32;

pub const DAY : days::Day<Day4Type1, Day4Type2> = days::Day {
    start1: 0,
    start2: 0,
    run1: &run1,
    run2: &run2,
    name: "day4",
    show1: &|x|{format!("{:?}", x)},
    show2: &|x|{format!("{:?}", x)},
};

fn run1 (line: &str, acc: Day4Type1) -> Day4Type1 {
    if line.is_empty() {
        return acc;
    }
    let re = Regex::new(r"^([0-9]+)\-([0-9]+),([0-9]+)\-([0-9]+)$").unwrap();
    let matches = re.captures(line).unwrap();
    let range1start = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let range1end = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let range2start = matches.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let range2end = matches.get(4).unwrap().as_str().parse::<u32>().unwrap();
    let range1outside = range1start <= range2start && range1end >= range2end;
    let range2outside = range2start <= range1start && range2end >= range1end;
    return if range1outside || range2outside { acc + 1 } else { acc };
}

fn run2 (line: &str, acc: Day4Type2) -> Day4Type2 {
    if line.is_empty() {
        return acc;
    }
    let re = Regex::new(r"^([0-9]+)\-([0-9]+),([0-9]+)\-([0-9]+)$").unwrap();
    let matches = re.captures(line).unwrap();
    let range1start = matches.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let range1end = matches.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let range2start = matches.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let range2end = matches.get(4).unwrap().as_str().parse::<u32>().unwrap();
    let range1starts_in_middle = range1start >= range2start && range1start <= range2end;
    let range1ends_in_middle = range1end >= range2start && range1end <= range2end;
    let range2starts_in_middle = range2start >= range1start && range2start <= range1end;
    let range2ends_in_middle = range2end >= range1start && range2end <= range1end;
    return if range1ends_in_middle || range1starts_in_middle || range2starts_in_middle || range2ends_in_middle { acc + 1 } else { acc };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run1 () {
        let res = run1("2-4,6-8", DAY.start1);
        let res = run1("2-3,4-5", res);
        let res = run1("5-7,7-9", res);
        let res = run1("2-8,3-7", res);
        let res = run1("6-6,4-6", res);
        let res = run1("2-6,4-8", res);
        let res = run1("12-120,1-13", res);
        
        assert!(res == 2, "{:?} did not equal 2", res);
    }

    #[test]
    fn test_run2 () {
        let res = run2("2-4,6-8", DAY.start2);
        let res = run2("2-3,4-5", res);
        let res = run2("5-7,7-9", res);
        let res = run2("2-8,3-7", res);
        let res = run2("6-6,4-6", res);
        let res = run2("2-6,4-8", res);
        
        assert!(res == 4, "{:?} did not equal 4", res);
    }
}