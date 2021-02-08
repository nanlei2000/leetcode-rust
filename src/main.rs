use std::collections::HashMap;

#[allow(dead_code)]
fn length_of_lis(nums: Vec<i32>) -> i32 {
  let len = nums.len();
  if len == 0 {
    return 0;
  }
  let mut dp: Vec<i32> = vec![1; len];
  let mut max_ans = 1;
  for i in 0..len {
    for j in 0..i {
      if nums[i] > nums[j] {
        dp[i] = dp[i].max(dp[j] + 1);
      }
    }
    max_ans = max_ans.max(dp[i])
  }
  return max_ans as i32;
}

fn length_of_longest_substring(s: String) -> i32 {
  let mut index_map: HashMap<char, usize> = HashMap::new();
  let mut start_index = 0;
  let mut max = 0;
  for (i, item) in s.char_indices() {
    let index = index_map.get(&item);
    if index.is_some() && index.unwrap() >= &start_index {
      let diff = i - start_index;
      if diff > max {
        max = diff;
      }
      start_index = index.unwrap() + 1;
    }
    index_map.insert(item, i);
  }
  let diff = s.len() - start_index;
  if diff > max {
    max = diff;
  }
  return max as i32;
}

fn main() {
  // let res = length_of_lis([10, 9, 2, 5, 3, 7, 101, 18].to_vec());
  let res = length_of_longest_substring("abba".to_string());
  println!("ans is {}", res);
}
