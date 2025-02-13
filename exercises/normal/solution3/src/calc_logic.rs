pub fn new_birthday_probability(n: u32) -> f64 {
    // 实现算法的思路就是按照365天，看相反的数  存在两个及以上，那么就可以求都没有的概率
    let years = 365;
    let mut p = 1.0_f64;
    for i in 1..=n {
        p = p * ((years - i + 1) as f64 / years as f64);
    }
    1.0_f64 - p
}
