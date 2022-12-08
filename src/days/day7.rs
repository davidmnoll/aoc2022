use regex::Regex;
use std::collections::HashSet;

use crate::days;



#[derive(Debug)]
#[derive(Clone)]
pub enum DirContents {
    Dir {
        name: String,
        contents: Vec<DirContents>
    },
    File {
        name: String,
        size: u32
    }
}




type Day7Type1 = (Vec<DirContents>, Vec<String>);
type Day7Type2 = u32;

pub const DAY : days::Day<Day7Type1, Day7Type2> = days::Day {
    puzzle1: days::Puzzle {
        start: (vec![], vec![]),
        run: &run1,
        show: &|x|{format!("{:?}", x)},
    },
    puzzle2: days::Puzzle {
        start:0, 
        run: &run2,
        show: &|x|{format!("{:?}", x)},
    },

    name: "day7",
};


fn run1(line: & str, acc: Day7Type1) -> Day7Type1 {
    if line.is_empty() {
        return acc;
    }
    let re_cmd_cd = Regex::new(r"^\$ cd ([\w\.\/])+\s*$").unwrap();
    let re_cmd_ls = Regex::new(r"^\$ ls\s*$").unwrap();
    let re_listing_file = Regex::new(r"^([0-9]+) ([\w\.]+)\s*$").unwrap();
    let re_listing_dir = Regex::new(r"^dir ([\w]+)\s*$").unwrap();
    let result = re_cmd_cd.captures(line).map(|x|{
        match x.get(1)?.as_str() {
            "/" => (acc.0, vec![]) ,
            ".." => {
                acc.1.remove(acc.1.len() - 1);
                (acc.0, acc.1) 
            },
            y => {
                acc.1.push(y);  
                (acc)
            },
            _ => None
        }
    });
    return if matches_cmd_cd.is_some() {
        (vec![DirContents::Dir {
            name: "test1".to_string(),
            contents: vec![]
        }], vec![])
    } else{
        acc
    }
}

fn run2 (line: &str, acc: Day7Type2) -> Day7Type2 {
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
        let start_val1 = (vec![
                DirContents::Dir {
                    name: "test1".to_string(), 
                    contents: vec![
                        DirContents::Dir {
                            name: "test2".to_string(), 
                            contents: vec![]
                        }, 
                        DirContents::File{ 
                            name: "file1".to_string(), 
                            size: 3
                        }
                    ]
                }
            ], vec![]);
        let res = run1("$ cd /", start_val1);
        let res = run1("$ ls", res);
        let res = run1("dir a", res);
        let res = run1("14848514 b.txt", res);
        let res = run1("8504156 c.dat", res);
        let res = run1("dir d", res);
        let res = run1("$ cd a", res);
        let res = run1("$ ls", res);
        let res = run1("dir e", res);
        let res = run1("29116 f", res);
        let res = run1("2557 g", res);
        let res = run1("62596 h.lst", res);
        let res = run1("$ ls", res);
        let res = run1("584 i", res);
        let res = run1("$ cd ..", res);
        let res = run1("$ cd ..", res);
        let res = run1("$ cd d", res);
        let res = run1("$ ls", res);
        let res = run1("4060174 j", res);
        let res = run1("8033020 d.log", res);
        let res = run1("5626152 d.ext", res);
        let res = run1("7214296 k", res);

        assert!(false, "({:?}, {:?}) not equal", res.0, res.1);
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