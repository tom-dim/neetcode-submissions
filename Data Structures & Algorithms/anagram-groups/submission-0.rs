use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        fn freq(word: &str) -> [u8; 26] {
            let mut count = [0u8; 26];
            for c in word.chars() {
                let idx = (c as u8 - b'a') as usize;
                count[idx] += 1;
            }
            count
        }

        let mut groups: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        for w in strs {
            groups.entry(freq(&w)).or_default().push(w);
        }
        groups.into_values().collect()
    }
}