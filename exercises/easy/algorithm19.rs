/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number.
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.

    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::{
    fmt::{self, Display, Formatter},
    ops::Mul,
};
#[derive(Clone, Debug)]
struct Matrix(Vec<Vec<i32>>);

pub fn fib(n: i32) -> i32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    fib_matrix(n) // Placeholder return value
}
// 使用递归的传统做法 时间复杂度是n^2
fn fib_digui(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib_digui(n - 1) + fib_digui(n - 2)
    }
}
/// 使用数组的方式实现 和递归一样，只是存在了数组里面
fn fib_vec(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let mut vec = vec![0; (n + 1) as usize];
    vec[0] = 0;
    vec[1] = 1;
    for i in 2..=n as usize {
        vec[i] = vec[i - 1] + vec[i - 2];
    }
    vec[n as usize]
}
/// Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
/// 矩阵幂的概念就是A^a   
/// [Fn,Fn-1] = A^n-1 [F1,F0]
/// A = [1 1]  A ^2 = [1 1] [1 1]   =  [2 1]    A^3  = [2 1] [1 1] = [3 2]
///     [1 0]          [1 0] [1 0]      [1 1]            [1 1] [1 0]  [2 1]
///
fn fib_matrix(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let matrix = Matrix(vec![vec![1, 1], vec![1, 0]]);
    // 计算出n-1次幂的矩阵 这里对n-1次幂进行计算的时候，走的是n/2 进行计算的，所以时间复杂度是logn
    let mattix_final = matrix_power(matrix, n - 1);
    // 与f1 f0计算
    let result = mattix_final * Matrix(vec![vec![1], vec![0]]);
    // 得出 fn  fn-1
    result.0[0][0]
}
// 进行降幂乘 logn的时间复杂度  这一步是精髓
fn matrix_power(mut matrix: Matrix, mut n: i32) -> Matrix {
    let mut result = Matrix(vec![vec![1, 0], vec![0, 1]]);
    //每一次都乘但是只有等于1 的时候才加上去
    while n > 0 {
        if n % 2 == 1 {
            result = result * matrix.clone();
        }
        matrix = matrix.clone() * matrix;
        n = n >> 1;
    }
    result
}
// 乘法
impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_a = self.0.len();
        let cols_a = self.0[0].len();
        let cols_b = rhs.0[0].len();
        let mut result = Matrix(vec![vec![0; cols_b]; rows_a]);
        // 进行矩阵乘法
        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a {
                    result.0[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
