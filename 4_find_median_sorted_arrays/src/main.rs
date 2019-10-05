// codes to submit begin
use std::cmp;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let a = nums1.len();
        let b = nums2.len();

        if a == 0 {
            if b & 1 != 0 {
                return nums2[b / 2] as f64;
            }
            return (nums2[b / 2] + nums2[b / 2 - 1]) as f64 / 2.0;
        }
        if b == 0 {
            if a & 1 != 0 {
                return nums1[a / 2] as f64;
            }
            return (nums1[a / 2] + nums1[a / 2 - 1]) as f64 / 2.0;
        }

        let s = a + b;
        if s & 1 != 0 {
            return Solution::find_kth(&nums1, 0, &nums2, 0, s / 2 + 1);
        }

        (Solution::find_kth(&nums1, 0, &nums2, 0, s / 2)
            + Solution::find_kth(&nums1, 0, &nums2, 0, s / 2 + 1)) / 2.0
    }

    fn find_kth(nums1: &Vec<i32>, begin1: usize, nums2: &Vec<i32>, begin2: usize
        , k: usize) -> f64 {
        if begin1 >= nums1.len() {
            return nums2[begin2 + k - 1] as f64;
        }
        if begin2 >= nums2.len() {
            return nums1[begin1 + k - 1] as f64;
        }
        if k == 1 {
            return cmp::min(nums1[begin1], nums2[begin2]) as f64;
        }

        // 递归查找
        let mut mid1 = i32::max_value();
        let mut mid2 = i32::max_value();
        if begin1 + k / 2 - 1 < nums1.len() {
            mid1 = nums1[begin1 + k / 2 - 1];
        }
        if begin2 + k / 2 - 1 < nums2.len() {
            mid2 = nums2[begin2 + k / 2 - 1];
        }
        if mid1 < mid2 {
            return Solution::find_kth(nums1, begin1 + k / 2, nums2, begin2, k - k / 2);
        }
        return Solution::find_kth(nums1, begin1, nums2, begin2 + k / 2, k - k / 2);
    }
}
// codes to submit end

// 两个有序数组求中位数，问题一般化为：求两个有序数组的第 k 个数，其中 k 为两个有序数组长度和的一半。
// find_kth 每次递归分别求出第一个和第二个数组的第 k / 2 个数 a 和 b，使问题规模缩小一半。
// 比较 a 和 b，当 a < b，说明第 k 个数位于第一个数组的后半段，或者第二个数组的前半段。
struct Solution {}

fn test(nums1: Vec<i32>, nums2: Vec<i32>, expect: f64) {
    assert_eq!(expect
        , Solution::find_median_sorted_arrays(nums1.to_vec(), nums2.to_vec())
        , "find_median_sorted_arrays({:?}, {:?})", nums1, nums2);
}

fn main() {
    test(vec![], vec![2], 2.0);
    test(vec![], vec![2, 3], 2.5);
    test(vec![1, 3, 4], vec![], 3.0);
    test(vec![1, 3], vec![], 2.0);

    test(vec![1, 3], vec![2], 2.0);
    test(vec![1, 2], vec![3, 4], 2.5);
    println!("PASS");
}
