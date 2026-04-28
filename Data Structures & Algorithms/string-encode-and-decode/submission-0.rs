impl Solution {
    pub fn encode(strs: Vec<String>) -> String {

        let mut result = String::new();

        for s in strs {   
            let len = s.chars().count();  
            result.push_str(&format!("{len}#{s}")); 
        } 
        result
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < s.len() {
            let hash = i + s[i..].find('#').unwrap();
            let len: usize = s[i..hash].parse().unwrap();
            let start = hash + 1;
            let end = start + len;
            result.push(s[start..end].to_string());
            i = end;
        }
        
        result
    }
}
