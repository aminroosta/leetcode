/* Container With Most Water
 * 
 * [Medium] [AC:50.6% 669.6K of 1.3M] [filetype:rust]
 * 
 * Given n non-negative integers a1, a2, ..., an , where each represents a
 * point at coordinate (i, ai). n vertical lines are drawn such that the two
 * endpoints of line i is at (i, ai) and (i, 0). Find two lines, which
 * together with x-axis forms a container, such that the container contains
 * the most water.
 * 
 * Note: You may not slant the container and n is at least 2.
 * 
 * The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In
 * this case, the max area of water (blue section) the container can contain
 * is 49. 
 * 
 * Example:
 * 
 * Input: [1,8,6,2,5,4,8,3,7]
 * 
 * Output: 49
 * 
 * [End of Description] */
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0; // iterator fram start
        let mut j = height.len() - 1; // iterator from end

        let mut best = 0; // best container
        while i != j {
            let min_ij = std::cmp::min(height[i], height[j]);
            best = std::cmp::max(best, (j-i) as i32 * min_ij);
            if height[i] == min_ij {
                i += 1;
            } else {
                j -= 1;
            }
        }
        best
    }
}
