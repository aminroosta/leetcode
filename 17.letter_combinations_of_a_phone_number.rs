impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        let mut res = vec![vec![]];
        let mut prev = vec![];
        for c in digits.chars() {
            let n = c as u8 - '0' as u8;
            std::mem::swap(&mut res, &mut prev);
            res.clear();
            let range = if n != 9 && n != 7 { 3 } else { 4 };
            for i in 0..range {
                let offset = if n > 7 { 1 } else { 0 };
                let ch = 'a' as u8 + (n-2)*3 + i + offset;
                for arr in prev.iter() {
                    let mut brr = arr.clone();
                    brr.push(ch);
                    res.push(brr);
                }
            }
        }

        res.iter().map(|v| String::from_utf8_lossy(v).into_owned()).collect::<_>()
    }
}
