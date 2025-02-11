/*
    Find Duplicates in Array
    Given an array, find all the duplicate elements and return them.
    You need to solve the problem with O(1) space complexity (i.e., without using extra arrays or hash tables).

    Implement the function `find_duplicates(nums: Vec<i32>) -> Vec<i32>`.
    The function should return a vector containing all the duplicate elements in the array.

    Hint: You can modify the input array in place to track duplicates.
*/
use std::fmt::{self, Display, Formatter};
/// o1的空间复杂度，不用额外的数组或者使用hashtable
pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    for i in 0..nums.len() {
        let stay_value = nums[i];
        if stay_value != 0 {
            nums[i] = 0;
            let mut is_repeat = false;
            for j in i..nums.len() {
                if nums[j] == stay_value && j != i {
                    nums[j] = 0;
                    is_repeat = true;
                }
            }
            if is_repeat {
                nums[i] = stay_value;
            }
        }
    }
    let mut nums = nums
        .iter()
        .filter(|item| **item != 0)
        .map(|item| *item)
        .collect::<Vec<i32>>();
    nums.sort();
    nums
    // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates_1() {
        let nums = vec![1, 2, 3, 4, 5, 6, 2, 3];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![2, 3]);
    }

    #[test]
    fn test_find_duplicates_2() {
        let nums = vec![4, 5, 6, 7, 5, 4];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn test_find_duplicates_3() {
        let nums = vec![1, 2, 3, 4, 5];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_find_duplicates_4() {
        let nums = vec![1, 1, 1, 1, 1];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_find_duplicates_5() {
        let nums = vec![10, 9, 8, 7, 6, 7, 8];
        let result = find_duplicates(nums);
        println!("Duplicates: {:?}", result);
        assert_eq!(result, vec![7, 8]);
    }
}
