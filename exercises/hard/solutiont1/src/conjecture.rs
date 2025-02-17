pub fn goldbach_conjecture() -> String {
    // 测试一下
    let mut count = 2;
    let mut number = 33;
    let mut sum = vec![];
    while count > 0 {
        if is_odd_composite(number) {
            if !odd_composite_number_to_prime_square(number) {
                sum.push(number.to_string());
                count = count - 1;
            }
        }
        number = number + 1;
    }
    return sum.join(",");
}
// 判断是否为素数
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    // 如果n能被整除或者3整除那就不是素数
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5u64;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i = i + 6;
    }
    return true;
}

fn odd_composite_number_to_prime_square(n: u64) -> bool {
    let temp = (n / 2) as f32;
    for i in 1..=temp.sqrt() as u64 {
        if is_prime(n - 2 * i * i) {
            return true;
        }
    }
    return false;
}
fn is_odd_composite(n: u64) -> bool {
    // 首先判断是否为奇数且大于1，因为合数是大于1的非素数
    if n <= 1 || n % 2 == 0 {
        return false;
    }

    // 判断是否为合数，即除了1和它自身外还有其他因数
    for i in 2..n {
        if n % i == 0 {
            return true;
        }
    }

    false
}
