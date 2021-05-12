use std::collections::HashMap;

#[allow(dead_code)]
fn length_of_longest_substring(s: String) -> i32 {
    let mut index_map: HashMap<char, usize> = HashMap::new();
    let mut start_index = 0;
    let mut max = 0;
    for (i, item) in s.char_indices() {
        match index_map.get(&item) {
            Some(index) => {
                if index >= &start_index {
                    let diff = i - start_index;
                    if diff > max {
                        max = diff;
                    }
                    start_index = index + 1;
                }
            }
            None => {}
        }
        index_map.insert(item, i);
    }
    let diff = s.len() - start_index;
    if diff > max {
        max = diff;
    }
    return max as i32;
}
#[test]
fn test_length_of_longest_substring() {
    let res = length_of_longest_substring("abba".to_string());
    println!("ans is {}", res);
}
