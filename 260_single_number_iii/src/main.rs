// https://leetcode-cn.com/problems/single-number-iii/

// codes to submit begin
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut bitmask = 0;
        for num in &nums {
            bitmask ^= num;
        }
        let diff = bitmask & (-bitmask);
        let mut x = 0;
        for num in &nums {
            if num & diff != 0 {
                x ^= num;
            }
        }
        vec![x, bitmask^x]
    }
}
// codes to submit end

struct Solution {}

fn test(nums: Vec<i32>, expect: Vec<i32>) {
    assert!(expect.len() == 2, "expect {:?} is invalid!", expect);

    let answer = Solution::single_number(nums.to_vec());
    assert!(answer.len() == 2
        && ((expect[0] == answer[0] && expect[1] == answer[1])
        || (expect[0] == answer[1] && expect[1] == answer[0]))
        , "single_number({:?}) = {:?}, expect {:?}", nums, answer, expect);
}

fn main() {
    test(vec![1,2,1,3,2,5], vec![3, 5]);
    println!("PASS");
}
