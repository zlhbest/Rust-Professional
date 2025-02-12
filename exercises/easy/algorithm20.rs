/*
    Sum of Two Integers
    Given two integers, calculate their sum without using the `+` operator.
    You need to implement the function `get_sum(a: i32, b: i32) -> i32`.
    The function should return the sum of the two integers `a` and `b`.

    Hint: You can solve this problem using bitwise operations.
*/

use std::fmt::{self, Display, Formatter};
/// 在不使用加号的时候
/// a与b的异或是处理的不同的值
/// a与b的与处理的相同的值，左移就是进位了
/// 在相加如果还有进位就再重复
pub fn get_sum(a: i32, b: i32) -> i32 {
    // TODO: Implement the logic to calculate the sum of two integers without using `+`
    // 异或是相同 = 0 不同 = 1 这就计算了当0和1的时候是1
    let mut sum = a ^ b;
    // a与b 算出来都是1的时候，如果 1 1 那结果就是1 将这个1左移，就代表进位
    let mut con = (a & b) << 1;
    while con != 0 {
        let temp = sum & con;
        sum = sum ^ con;
        con = temp << 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_1() {
        let result = get_sum(1, 2);
        println!("Sum of 1 and 2: {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sum_2() {
        let result = get_sum(-1, 1);
        println!("Sum of -1 and 1: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_3() {
        let result = get_sum(100, 200);
        println!("Sum of 100 and 200: {}", result);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_sum_4() {
        let result = get_sum(-50, -50);
        println!("Sum of -50 and -50: {}", result);
        assert_eq!(result, -100);
    }

    #[test]
    fn test_sum_5() {
        let result = get_sum(0, 0);
        println!("Sum of 0 and 0: {}", result);
        assert_eq!(result, 0);
    }
}
