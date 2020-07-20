impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let rows = num_rows as usize;
        if rows == 1 {
            return s;
        }

        let mut res = vec![];
        for i in 0..rows {
            let mut j = i;
            while j < n {
                res.push(bytes[j]);
                if i > 0 && i < rows - 1 {
                    let m = j + 2 * (rows - 1 - i);
                    if m < n {
                        res.push(bytes[m]);
                    }
                }
                j += 2 * (rows - 1);
            }
        }

        String::from_utf8(res).unwrap()
    }
}
