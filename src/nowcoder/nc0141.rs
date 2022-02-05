struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    *
        * @param str string字符串 待判断的字符串
        * @return bool布尔型
     */
    pub fn judge(&self, str: String) -> bool {
        let mut chars: Vec<char> = str.chars().collect();
        let len: usize = chars.len();
        let mut i: usize = 0;
        while true {
            let corresponding_val = len - 1 - i;
            if corresponding_val <= i {
                break;
            }
            if chars[i] != chars[corresponding_val] {
                return false;
            }
            i += 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::new().judge("absba".to_owned());
        println!("{}", result)
    }

    #[test]
    fn test_case2() {
        let result = Solution::new().judge("aaaaaaa"
            .to_owned());
        println!("{}", result)
    }
}