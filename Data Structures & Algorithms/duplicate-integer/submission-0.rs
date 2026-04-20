use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {

    let mut dupes = HashSet::new();

    for dupe in nums {
        if dupes.contains(&dupe) {
        return true;
        }
    dupes.insert(dupe);
    }
    false

    }
}
