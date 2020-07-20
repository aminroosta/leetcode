impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result = 0;
        let mut y = x;
        while y != 0 {
            // check for overflow
            if result != (result * 10 / 10) { return 0; }
            result = result * 10 + y % 10;
            y /= 10;
        }
        result
    }
}
