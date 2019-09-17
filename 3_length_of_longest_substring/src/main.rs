// codes to submit begin
use std::collections::VecDeque;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut ss: VecDeque<(char)> = VecDeque::with_capacity(s.chars().count());
        for c in s.chars() {
            if ss.contains(&c) {
                loop {
                    if ss.pop_front() == Some(c) {
                        break;
                    }
                }
                ss.push_back(c);
            } else {
                ss.push_back(c);
                if max < ss.len() {
                    max = ss.len();
                }
            }
        }
        max as i32
    }
}
// codes to submit end

// UMU: slow, s.chars() copy every time
// impl Solution {
//     pub fn length_of_longest_substring(s: String) -> i32 {
//         let mut max = 0;
//         let mut lower = 0;
//         let mut upper = 0;
//         let mut i = 0;
//         let count = s.chars().count();
//         while i < count && lower + max < count {
//             let mut p = lower;
//             while s.chars().nth(p) != s.chars().nth(i) && p < upper {
//                 p += 1;
//             }
//             if p < upper {
//                 lower = p + 1;
//                 upper += 1;
//             } else {
//                 upper += 1;
//                 if max < upper - lower {
//                     max = upper - lower;
//                 }
//             }
//             i += 1;
//         }

//         max as i32
//     }
// }

struct Solution {}

fn test(s: &str, expect: i32) {
    assert_eq!(expect, Solution::length_of_longest_substring(s.to_string())
        , "length_of_longest_substring(\"{}\")", s);
}

fn main() {
    test("abcabcbb", 3);
    test("bbbbb", 1);
    test("pwwkew", 3);
    test("", 0);
    test("bwf", 3);  

    println!("PASS");
}
