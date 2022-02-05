struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    *
        * @param str string字符串
        * @return int整型
     */
    pub fn FirstNotRepeatingChar(&self, str: String) -> i32 {
        let mut array = vec![0; 255];
        let mut aux = 0;
        for byte in str.bytes() {
            array[byte as usize] += 1;
            aux += 1;
        }
        let mut aux = 0;
        for byte in str.bytes() {
            if array[byte as usize] == 1 {
                return aux;
            }
            aux += 1;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::new().FirstNotRepeatingChar("google".to_owned());
        println!("{}", result)
    }
}