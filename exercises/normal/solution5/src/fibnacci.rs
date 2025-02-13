pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    // 第一步求出小于n的斐波那契数
    let mut array = Vec::new();
    fibnacci(threshold, &mut array);
    let mut sum = 0;
    for item in array {
        if item % 2 == 1 {
            sum = sum + item;
        }
    }
    sum
}

fn fibnacci(n: u32, array: &mut Vec<u32>) {
    array.insert(0, 0);
    array.insert(1, 1);
    let mut index = 2;
    loop {
        let next = array.get(index - 1).unwrap() + array.get(index - 2).unwrap();
        if next < n {
            array.insert(index, next);
            index = index + 1;
        } else {
            break;
        }
    }
}
