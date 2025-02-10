/*
    Anagram Check
    Given two strings, check if they are anagrams of each other.
    Anagrams are words or phrases formed by rearranging the letters of another,
    using all the original letters exactly once.
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::fmt::{self, Display, Formatter};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    let mut s1_vec = s1
        .to_lowercase()
        .chars()
        .filter(|item| *item != ' ')
        .map(|item| item as usize)
        .collect::<Vec<usize>>();
    let mut s2_vec = s2
        .to_lowercase()
        .chars()
        .filter(|item| *item != ' ')
        .map(|item| item as usize)
        .collect::<Vec<usize>>();
    if s1_vec.len() != s2_vec.len() {
        return false;
    }
    s1_vec.sort();
    s2_vec.sort();
    for i in 0..s1_vec.len() {
        if s1_vec[i] != s2_vec[i] {
            return false;
        }
    }

    true // Placeholder return value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
