impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut y = x;
        let mut xr = 0; // x-reversed
        while y != 0 {
            xr = xr * 10 + y % 10;
            y  = y / 10;
        }

        return x >= 0 && x == xr;
    }
}
