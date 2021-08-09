pub struct Solution {}

impl Solution {
    pub fn is_majority_element(nums: Vec<i32>, target: i32) -> bool {
        let left = Self::find_val_left(nums.as_slice(), target);
        if left == -1 {
            return false;
        }
        let half = nums.len() as i32 / 2;
        let right = Self::find_val_right(nums.as_slice(), target);
        return (right - left + 1) > half;
    }

    // return -1 if not exists
    pub fn find_val_left(nums: &[i32], target: i32) -> i32 {
        let size = nums.len();
        let mut left: i32 = 0;
        let mut right: i32 = size as i32 - 1;
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid as usize] >= target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        if nums[right as usize] == target {
            return right;
        }
        return -1;
    }

    // return -1 if not exists
    pub fn find_val_right(nums: &[i32], target: i32) -> i32 {
        let size = nums.len();
        let mut left: i32 = 0;
        let mut right: i32 = size as i32 - 1;
        while left < right {
            let mid = (left + right + 1) / 2;
            if nums[mid as usize] <= target {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        if nums[left as usize] == target {
            return left;
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let result = Solution::is_majority_element(vec![2, 4, 5, 5, 5, 5, 5, 6, 6], 5);
        assert_eq!(true, result);
    }
}