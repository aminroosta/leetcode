impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut result = vec![];

        for i in 0..nums.len() {
            for j in (i+1)..nums.len()-1 {
                let sum = nums[i] + nums[j];
                if nums[j+1..].binary_search(&(sum * -1)).is_ok() {
                    let mut v = vec![nums[i], nums[j], sum * -1];
                    if result.binary_search(&v).is_err() {
                        result.push(v);
                    }
                }
            }
        }
        result
    }
}
