/*
    Palindrome Check
    Given a string, check if it is a palindrome (i.e., it reads the same forward and backward).
    The solution should ignore case differences and non-alphabetical characters.

    You need to implement the function `is_palindrome(s: String) -> bool`.
    The function should return `true` if the string is a palindrome, and `false` otherwise.

    Hint: Consider normalizing the string by converting it to lowercase and removing non-alphabetical characters before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn is_palindrome(s: String) -> bool {
    // TODO: Implement the logic to check if the string is a palindrome
    let chars = s.chars().collect::<Vec<char>>();
    let (mut left, mut right) = (0, chars.len() - 1);
    while left <= right {
        while !is_char(chars[left]) {
            left += 1;
        }
        while !is_char(chars[right]) {
            right -= 1;
        }
        if left <= right {
            if chars[left].to_lowercase().to_string() == chars[right].to_lowercase().to_string() {
                left += 1;
                right -= 1;
            } else {
                return false;
            }
        }
    }
    true // Placeholder return value
}

fn is_char(az: char) -> bool {
    let char_number = az as usize;
    (97 <= char_number && char_number <= 122) || (char_number >= 65 && char_number <= 90)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
