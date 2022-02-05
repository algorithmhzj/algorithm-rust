struct Solution{

}

impl Solution {
    fn new() -> Self {
        Solution{}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    *
    *
        * @param s string字符串
        * @return string字符串
     */
    pub fn replaceSpace(&self, s: String) -> String {
        return s.replace(" ", "%20")
    }
}