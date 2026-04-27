use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {

        let mut freq = HashMap::new();
        let n_len = nums.len();

        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }

        let mut buckets: Vec<Vec<i32>> = vec![Vec::new(); n_len + 1];
        for (num, count) in freq {
            buckets[count as usize].push(num);
        }

        buckets.into_iter()
            .rev()
            .flatten()
            .take(k as usize)
            .collect()
    }
}
