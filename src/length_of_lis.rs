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

#[test]
pub fn test_length_of_lis() {
    let res = length_of_lis([10, 9, 2, 5, 3, 7, 101, 18].to_vec());
    println!("ans is {}", res);
}
