use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut seen = HashMap::new();

        for (i,x) in nums.iter().enumerate() {
            if seen.contains_key(&(target - x)) {
                let value = *seen.get(&(target - x)).unwrap() as i32;
                return vec![
                    value, i as i32
                    ];
            }
        seen.insert(*x,i);
        }
        vec![]
    }
}
