// https://leetcode-cn.com/problems/zigzag-conversion/

// codes to submit begin
use std::iter::FromIterator;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        if s.len() < 2 || num_rows < 2 || s.len() <= num_rows {
            return s;
        }

        let mut counts: Vec<usize> = vec![0; num_rows];
        let mut r = 0;
        let mut l = 0;
        let mut reversed = true;
        while l < s.len() {
            counts[r] += 1;
            if r == 0 || r == num_rows - 1 {
                reversed = !reversed;
            }
            if reversed {
                r -= 1;
            } else {
                r += 1;
            }
            l += 1;
        }

        l = counts[0];
        counts[0] = 0;
        r = 1;
        while r < num_rows {
            let t = counts[r];
            counts[r] = l;
            l += t;
            r += 1;
        }

        let mut pos: Vec<usize> = vec![0; num_rows];
        let mut result: Vec<char> = vec!['\0'; s.len()];
        r = 0;
        reversed = true;
        for c in s.chars() {
            result[counts[r] + pos[r]] = c;
            pos[r] += 1;
            if r == 0 || r == num_rows - 1 {
                reversed = !reversed;
            }
            if reversed {
                r -= 1;
            } else {
                r += 1;
            }
        }
        String::from_iter(result)
    }
}
// codes to submit end

// NOT FAST ENOUGH
// impl Solution {
//     pub fn convert(s: String, num_rows: i32) -> String {
//         let num_rows = num_rows as usize;
//         if s.len() < 2 || num_rows < 2 || s.len() <= num_rows {
//             return s;
//         }

//         let mut v: Vec<Vec<char>> = Vec::with_capacity(num_rows);
//         v.resize(num_rows, Vec::with_capacity(s.len()));

//         let mut i = 0;
//         let mut reversed = true;
//         for c in s.chars() {
//             v[i].push(c);
//             if i == 0 || i == num_rows - 1 {
//                 reversed = !reversed;
//             }
//             if reversed {
//                 i -= 1;
//             } else {
//                 i += 1;
//             }
//         }

//         let mut result = String::new();
//         for e in v {
//             result += &String::from_iter(e);
//         }
//         result
//     }
// }

// NOT FAST ENOUGH
// impl Solution {
//     pub fn convert(s: String, num_rows: i32) -> String {
//         if s.len() < 2 || num_rows < 2 {
//             return s;
//         }

//         let v: Vec<char> = s.chars().collect();
//         let mut result: Vec<char> = Vec::with_capacity(v.len());
//         let mut row: usize = 0;
//         let num_rows = num_rows as usize;
//         while row < num_rows {
//             let mut i = row;
//             let mut even = if row < num_rows - 1 { false } else { true };
//             while i < v.len() {
//                 result.push(v[i]);
//                 if even {
//                     i += row * 2;
//                 } else {
//                     i += (num_rows - row - 1) * 2;
//                 }
//                 if row > 0 && row < num_rows - 1 {
//                     even = !even;
//                 }
//             }
//             row += 1;
//         }

//         String::from_iter(result)
//     }
// }

// Slow
// impl Solution {
//     pub fn convert(s: String, num_rows: i32) -> String {
//         if s.len() < 2 || num_rows < 2 {
//             return s;
//         }

//         let mut result: Vec<char> = vec!['\0'; s.len()];
//         let num_rows = num_rows as usize;
//         let n = num_rows * 2 - 2;
//         let num_column = s.len() / n;
//         let mut remainder = s.len() % n;
//         let mut counts: Vec<usize> = vec![0; num_rows];
//         counts[0] = num_column;
//         let mut i = 1;
//         while i < num_rows - 1 {
//             counts[i] = num_column * 2;
//             i += 1;
//         }
//         counts[i] = num_column;
//         i = 0;
//         let mut reversed = false;
//         while remainder > 0 {
//             counts[i] += 1;
//             if i < num_rows - 1 && !reversed {
//                 i += 1;
//             } else {
//                 reversed = true;
//                 i -= 1;
//             }
//             remainder -= 1;
//         }
//         for (i, c) in s.char_indices() {
//             let mut p = 0;
//             let mut r = 0;
//             let row = i % n;
//             let (row, over) = if row < num_rows { (row, 0) } else { (n - row, 1) };
//             while r < row {
//                 p += counts[r];
//                 r += 1;
//             }
//             if row > 0 && row < num_rows - 1 {
//                 p += (i / n) * 2 + over;
//             } else {
//                 p += i / n;
//             }
//             result[p] = c;
//         }

//         String::from_iter(result)
//     }
// }

struct Solution {}

fn test(s: &str, num_rows: i32, expect: &str) {
    assert_eq!(expect, Solution::convert(s.to_string(), num_rows)
        , "convert({:?}, {:?})", s, num_rows);
}

fn main() {
    test("U", 2, "U");
    test("LEETCODEISHIRING", 1, "LEETCODEISHIRING");
    test("UMU", 2, "UUM");
    test("LEETCODEISHIRING", 3, "LCIRETOESIIGEDHN");
    test("LEETCODEISHIRING", 4, "LDREOEIIECIHNTSG");
    test("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR");
    test("PAYPALISHIRING", 9, "PAYPGANLIIRSIH");
    println!("PASS");
}
