use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        if strs.len() == 1 {
            return strs[0].clone();
        }
        let mut prefix = "".to_string();

        for (index, char) in strs[0].chars().enumerate() {
            for (j, item) in strs[1..].iter().enumerate() {
                if item.chars().nth(index) != Some(char) {
                    return prefix;
                }
                if j == strs.len() - 2 {
                    prefix += &char.to_string();
                }
            }
        }
        prefix
    }
}

#[test]
fn test_longest_common_prefix() {
    let res = Solution::longest_common_prefix(["a".to_string(), "aaab".to_string()].to_vec());
    println!("ans is {}", res);
}
