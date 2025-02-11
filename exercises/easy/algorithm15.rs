/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters.
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.

    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::{
    cmp::max,
    fmt::{self, Display, Formatter},
};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // TODO: Implement the logic to find the longest substring without repeating characters
    if s.is_empty() {
        return 0;
    }
    let (mut max_length, mut longest_size) = (1, 1);
    let (mut left, mut right) = (0, 1);
    // 使用一个滑动窗口实现
    let mut sub_string = &s[left..right];
    while right < s.len() {
        let mut char_index = &s[right..=right];
        // 往外阔
        while !sub_string.contains(char_index) {
            right += 1;
            sub_string = &s[left..right];
            longest_size += 1;
            max_length = max(max_length, longest_size);
            if right < s.len() {
                char_index = &s[right..=right];
            } else {
                break;
            }
        }
        // 发现不行就收缩left
        while left < right {
            if sub_string.contains(char_index) {
                left += 1;
                longest_size -= 1;
                sub_string = &s[left..right];
            } else {
                break;
            }
        }
    }

    max_length // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1); // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0); // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5); // "abcde"
    }
}
