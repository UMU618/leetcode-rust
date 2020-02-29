// https://leetcode-cn.com/problems/majority-element/

// codes to submit begin
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut i = 1;
        let mut count = 1;
        while i < nums.len() {
            if result == nums[i] {
                count += 1;
                if count > nums.len() / 2 {
                    break;
                }
            } else {
                count -= 1;
            }
            if count == 0 {
                i += 1;
                count = 1;
                result = nums[i];
            }
            i += 1;
        }
        result
    }
    // No 1
    // pub fn majority_element(nums: Vec<i32>) -> i32 {
    //     let mut arr = nums;
    //     arr.sort();
    //     arr[arr.len() / 2]
    // }

    // No 2
    // pub fn majority_element(nums: Vec<i32>) -> i32 {
    //     let length = nums.len();
    //     let mut removed: Vec<bool> = vec![false; length];
    //     let mut a: usize = 0;
    //     loop {
    //         let mut b = Solution::next(a, &removed);
    //         if b == length {
    //             break;
    //         }
    //         let mut remove = false;
    //         loop {
    //             if nums[a] != nums[b] {
    //                 removed[a] = true;
    //                 removed[b] = true;
    //                 remove = true;
    //                 break;
    //             }
    //             b = Solution::next(b, &removed);
    //             if b == length {
    //                 break;
    //             }
    //         }
    //         if !remove {
    //             break;
    //         }
    //         b = Solution::next(a, &removed);
    //         if b == length {
    //             break;
    //         }
    //         a = b;
    //     }
    //     nums[a]
    // }

    // fn next(mut pos: usize, removed: &Vec<bool>) -> usize {
    //     pos += 1;
    //     while pos < removed.len() {
    //         if !removed[pos] {
    //             break;
    //         }
    //         pos += 1;
    //     }
    //     return pos;
    // }
}
// codes to submit end

struct Solution {}

fn test(nums: Vec<i32>, expect: i32) {
    assert_eq!(expect, Solution::majority_element(nums.to_vec())
        , "majority_element({:?})", nums);
}

fn main() {
    test(vec![3,2,3], 3);
    test(vec![2,2,1,1,1,2,2], 2);
    test(vec![6,5,5], 5);
    println!("PASS");
}
