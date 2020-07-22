impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut best = 1000 * 1000;

        for i in 0..nums.len() {
            for j in i+1..nums.len()-1 {
                let a = nums[i];
                let b = nums[j];
                match nums[j+1..].binary_search(&(target - a - b)) {
                    Ok(_) => { return target; },
                    Err(k) => {
                        let k = k + j + 1;
                        if k > j + 1 {
                            let sum = a + b + nums[k-1];
                            if (sum - target).abs() < (best - target).abs() {
                                best = sum;
                            }
                        }
                        if k < nums.len() {
                            let sum = a + b + nums[k];
                            if (sum - target).abs() < (best - target).abs() {
                                best = sum;
                            }
                        }
                    }
                }
            }
        }
        best
    }
}
