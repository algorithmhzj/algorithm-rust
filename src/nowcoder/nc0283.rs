use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn new() -> Self {
        Solution {}
    }

    /**
    * 代码中的类名、方法名、参数名已经指定，请勿修改，直接返回方法规定的值即可
    *
    *
        * @param numbers int整型一维数组
        * @return int整型
     */
    pub fn duplicate(&self, numbers: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for number in numbers {
            if set.contains(&number) {
                return number;
            }
            set.insert(number);
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::new().duplicate(vec![1, 2, 3]);
        println!("{}", result)
    }

    #[test]
    fn test_case2() {
        let result = Solution::new().duplicate(vec![2, 3, 1, 0, 2, 5, 3]);
        println!("{}", result)
    }
}
