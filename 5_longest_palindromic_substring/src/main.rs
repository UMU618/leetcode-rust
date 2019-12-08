// https://leetcode-cn.com/problems/longest-palindromic-substring/
//use std::iter::FromIterator;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string()
        }

        let v: Vec<char> = s.chars().collect();
        let len = v.len();
        let mut max = 0;
        let mut max_lbound = 0;
        let mut max_rbound = 0;
        let mut i = 0;
        while i < len {
            let mut lbound = i;
            let mut rbound = i;
            while rbound + 1 < len && v[lbound] == v[rbound + 1] {
                rbound += 1;
            }
            i += rbound - lbound + 1;

            while lbound > 0 && rbound + 1 < len
                && v[lbound - 1] == v[rbound + 1] {
                lbound -= 1;
                rbound += 1;
            }

            let count = rbound - lbound + 1;
            if count > max {
                max = count;
                max_lbound = lbound;
                max_rbound = rbound;
            }

            if max >= (len - i) * 2 - 1 {
                break;
            }
        }
        v[max_lbound..max_rbound + 1].iter().collect()
    }
}

struct Solution {}

fn test(s: &str, expect: &str) {
    assert_eq!(expect, Solution::longest_palindrome(s.to_string())
        , "longestPalindrome(\"{}\")", s);
}

fn main() {
    test("babad", "bab");
    test("cbbd", "bb");
    test("abacab", "bacab");
}
