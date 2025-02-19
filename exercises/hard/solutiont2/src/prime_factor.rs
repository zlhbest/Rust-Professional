use rand::random;

/// 用于找出正整数的最大素数因子
/// 素数的定义是只能被1和他本身整除
/// 重写 https://oi-wiki.org/math/number-theory/pollard-rho/
/// gcd是最大公约数
/// 费马小定理： 若p为素数，gcd(a,p) = 1 则 a^p-1 == 1(mod p)
///  
pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut max_factor = 0;
    fac(number, &mut max_factor);
    max_factor
}
fn fac(mut x: u128, factor: &mut u128) {
    if x <= *factor || x < 2 {
        return;
    }
    if is_prime(x) {
        // 如果x为质数
        *factor = std::cmp::max(*factor, x); // 更新答案
        return;
    }
    let mut p = x;
    while p >= x {
        p = pollard_rho(x); // 使用该算法
    }
    while (x % p) == 0 {
        x /= p;
    }
    fac(x, factor);
    fac(p, factor); // 继续向下分解x和p
}

/// 使用pollard_rho算法快速计算
fn pollard_rho(x: u128) -> u128 {
    let mut t = 0;
    let c = random::<u128>() % (x - 1) + 1;
    let mut val = 1;
    for len in std::iter::successors(Some(2), |x| Some(x * 2)) {
        let s = t;
        for step in 1..=len {
            t = (ksc(t, t, x) + c) % x;
            val = ksc(val, t.abs_diff(s), x);
            if step % 127 == 0 {
                let d = gcd(val, x);
                if d > 1 {
                    return d;
                }
            }
        }
        let d = gcd(val, x);
        if d > 1 {
            return d;
        }
    }
    unreachable!()
}
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}
/// 使用miller_rabin判断是否为质数，该方法原理我没看懂，只是照着写出来了
fn is_prime(x: u128) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 || x == 3 || x == 5 || x == 7 || x == 43 {
        return true;
    }
    miller_rabin(2, x) && miller_rabin(3, x) && miller_rabin(5, x) && miller_rabin(43, x)
}
/// miller_rabin 判断质数
fn miller_rabin(x: u128, p: u128) -> bool {
    if ksm(x, p - 1, p) != 1 {
        return false;
    }
    let mut y = p - 1;
    let mut z;
    while y & 1 == 0 {
        y >>= 1;
        z = ksm(x, y, p);
        if z != 1 && z != p - 1 {
            return false;
        }
        if z == p - 1 {
            return true;
        }
    }
    return true;
}
/// 快速乘
fn ksc(mut x: u128, mut y: u128, p: u128) -> u128 {
    let mut ans = 0;
    while y > 0 {
        if y & 1 == 1 {
            ans = (ans + x) % p;
        }
        x = (x + x) % p;
        y >>= 1;
    }
    ans
}
/// 快速幂
fn ksm(mut x: u128, mut y: u128, p: u128) -> u128 {
    let mut res: u128 = 1;
    while y != 0 {
        if y & 1 != 0 {
            res = ksc(res, x, p);
        }
        x = ksc(x, x, p);
        y >>= 1;
    }
    res
}

#[cfg(test)]
mod test {
    use crate::prime_factor::find_max_prime_factor;

    use super::is_prime;

    #[test]
    fn is_prime_test() {
        assert_eq!(true, is_prime(5218879));
    }
    #[test]
    fn find_max_prime_factor_test() {
        assert_eq!(
            203729729563409477,
            find_max_prime_factor(97993999919999958437)
        );
    }
}
