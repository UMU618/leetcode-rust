struct Solution {
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::BTreeMap::new();
        for (k, v) in nums.iter().enumerate() {
            let another = target - *v;
            if let Some(num) = map.get(&another) {
                return vec![*num as i32, k as i32];
            }
            map.insert(v, k);
        }
        vec![]
    }
}

fn main() {
    let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
    if vec![0, 1] == result {
        println!("PASS");
    } else {
        println!("FAILED, wrong answer: {:?}", result);
    }
}
