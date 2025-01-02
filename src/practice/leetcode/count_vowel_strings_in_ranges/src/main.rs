// https://leetcode.com/problems/count-vowel-strings-in-ranges

struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let is_vowel =
            |c: char| -> bool { c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' };
        let mut cnt: Vec<i32> = vec![];
        for w in words.iter() {
            let mut c_set = w.chars();
            let f = c_set.nth(0).unwrap();
            let s = if w.len() == 1 {
                f
            } else {
                c_set.nth(w.len() - 2).unwrap()
            };

            if is_vowel(f) && is_vowel(s) {
                cnt.push(1);
            } else {
                cnt.push(0);
            }
        }
        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1]
        }

        let mut ans: Vec<i32> = vec![];
        for v in queries.iter() {
            let i = v[0];
            let j = v[1];
            let mut ret = cnt[j as usize];
            if i > 0 {
                ret -= cnt[(i - 1) as usize];
            }
            ans.push(ret);
        }

        ans
    }
}

fn main() {
    let result = Solution::vowel_strings(
        vec![
            "aba".to_string(),
            "bcb".to_string(),
            "ece".to_string(),
            "aa".to_string(),
            "e".to_string(),
        ],
        vec![vec![0, 2], vec![1, 4], vec![1, 1]],
    );
    print!("{:?}", result)
}
