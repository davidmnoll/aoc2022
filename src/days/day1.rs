use crate::days;

pub fn run1(line: &str, acc: (Vec<i32>, i32)) -> (Vec<i32>, i32){
    return if line.is_empty(){
        let my_sum = acc.0.into_iter().sum();
        if  my_sum > acc.1 {
            (vec![], my_sum)
        } else {
            (vec![], acc.1)
        }
    }else{
        ([acc.0, vec![line.parse::<i32>().unwrap()]].concat(), acc.1)
    }
}
pub fn run2(line: &str, acc: (Vec<i32>, (i32, i32, i32))) -> (Vec<i32>, (i32, i32, i32)){
    return if line.is_empty(){
        let my_sum = acc.0.into_iter().sum();
        if  my_sum > acc.1.0 {
            (vec![], (my_sum, acc.1.0, acc.1.1))
        } else if my_sum > acc.1.1 {
            (vec![], (acc.1.0, my_sum, acc.1.1))
        } else if my_sum > acc.1.2 {
            (vec![], (acc.1.0, acc.1.1, my_sum))
        } else{
            (vec![], acc.1)
        }
    }else{
        ([acc.0, vec![line.parse::<i32>().unwrap()]].concat(), acc.1)
    }
}





type Day1Type1 = (Vec<i32>, i32);
type Day1Type2 = (Vec<i32>, (i32, i32, i32));

pub const DAY : days::Day<Day1Type1, Day1Type2> = days::Day {
    start1: ( vec![], i32::MIN),
    start2:  ( vec![], (i32::MIN, i32::MIN, i32::MIN)),
    run1: &run1,
    run2: &run2,
    name: "day1"
};
        // self.reduce_input_lines(self.run1, ( vec![], i32::MIN));
        // self.reduce_input_lines(self.run2, ( vec![], (i32::MIN, i32::MIN, i32::MIN)));
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run1_runs() {
        let res0 = run1("35", (vec![12, 14], 20));
        let res1 = run1("", res0);
        let res2 = run1("3", res1);
        println!("res2.0 {:?}", res2.0);
        println!("res2.1 {:?}", res2.1);
        assert!(res2.1 == 61);
    }


    #[test]
    fn run2_runs() {
        let mut res = run2("35", (vec![12, 14], (20, 21, 23)));
        res = run2("", res);
        res = run2("31", res);
        res = run2("32", res);
        res = run2("33", res);
        res = run2("", res);
        res = run2("35", res);
        res = run2("36", res);
        res = run2("20", res);
        res = run2("", res);
        res = run2("14", res);
        res = run2("36", res);
        res = run2("20", res);
        res = run2("", res);
        res = run2("54", res);
        res = run2("56", res);
        res = run2("50", res);
        res = run2("", res);
        println!("res2.0 {:?}", res.0);
        println!("res2.1 {:?}", res.1);
        assert!(res.1.0 == 160 && res.1.1 == 96 && res.1.2 == 91);
    }
}