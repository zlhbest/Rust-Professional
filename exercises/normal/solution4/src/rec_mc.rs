const CASH_VALUE: &[u32] = &[1, 2, 5, 10, 20, 30, 50, 100];
pub fn dp_rec_mc(amount: u32) -> u32 {
    // 这里明显就是使用贪心来处理
    let mut money_value = amount;
    let mut count = 0;
    while money_value > 0 {
        // 找需要的钱
        let mut max_money = 0;
        for i in CASH_VALUE {
            if *i <= money_value {
                if *i > max_money {
                    max_money = *i;
                }
            }
        }
        count = count + 1;
        money_value = money_value - max_money;
    }
    count
}
