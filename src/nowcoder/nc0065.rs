struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    *
        * @param n int整型
        * @return int整型
     */
    pub fn Fibonacci(&self, n: i32) -> i32 {
        let array: [i32; 41] = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584,
            4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040
            , 1346269, 2178309, 3524578, 5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155];
        return array[n as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::new().Fibonacci(1);
        println!("{}", result)
    }

}