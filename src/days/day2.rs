// pub fn run1() {

// }

// pub fn run2() {

// }
// pub static DAY: Day = Day {
//     name: Box::new("day2"),
//     fn1: Box::new(run1), 
//     fn2: Box::new(run2), 
//     val1: (), 
//     val2: ()
// };


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn run1_runs() {
//         let res0 = run1("35", (vec![12, 14], 20));
//         let res1 = run1("", res0);
//         let res2 = run1("3", res1);
//         println!("res2.0 {:?}", res2.0);
//         println!("res2.1 {:?}", res2.1);
//         assert!(res2.1 == 61);
//     }


//     #[test]
//     fn run2_runs() {
//         let mut res = run2("35", (vec![12, 14], (20, 21, 23)));
//         res = run2("", res);
//         res = run2("31", res);
//         res = run2("32", res);
//         res = run2("33", res);
//         res = run2("", res);
//         res = run2("35", res);
//         res = run2("36", res);
//         res = run2("20", res);
//         res = run2("", res);
//         res = run2("14", res);
//         res = run2("36", res);
//         res = run2("20", res);
//         res = run2("", res);
//         res = run2("54", res);
//         res = run2("56", res);
//         res = run2("50", res);
//         res = run2("", res);
//         println!("res2.0 {:?}", res.0);
//         println!("res2.1 {:?}", res.1);
//         assert!(res.1.0 == 160 && res.1.1 == 96 && res.1.2 == 91);
//     }
// }