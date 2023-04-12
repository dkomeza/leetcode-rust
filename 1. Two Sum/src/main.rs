fn main() {
    println!("Hello, world!");
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, number) in nums.iter().enumerate() {
        for index in i + 1..nums.len() {
            if nums[index] == target - number {
                return vec![i as i32, index as i32];
            }
        }
    }
    return vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_sum() {
        assert_eq!(super::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
