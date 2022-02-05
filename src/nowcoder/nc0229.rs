struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    *
    *
        * @param str string字符串
        * @return bool布尔型
     */
    pub fn isUnique(&self, str: String) -> bool {
        let mut array = vec![0; 255];
        for byte in str.bytes() {
            array[byte as usize] += 1;
            if array[byte as usize] > 1 {
                return false;
            }
        }
        return true;
    }
}