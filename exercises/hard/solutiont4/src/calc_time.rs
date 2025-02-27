use time_utils::{
    chinese_date::ChineseDate, date::Date, holiday::days_to_trading, utils::all_days_in_year,
};

pub fn time_info(time: &str) -> String {
    let date: Date = time.into();
    let days = date.days_in_year();
    format!(
        "{},{},{},{},{},{}",
        date.week_of_year(days),            // 第几个周
        date.day_of_week(),                 // 周几
        days,                               // 过了多少天
        all_days_in_year(date.year) - days, // 还剩多少天
        chinese_days(&date),                // 距离春节还有多少天
        days_to_trading(&date),             //距离股市还有多少天
    )
}
fn chinese_days(date: &Date) -> u32 {
    // 获取chinese date
    let mut chinese_date = ChineseDate::new(date.year, 1, 1).lunar_to_solar();
    let timetemp = date.date_to_timetemp();
    if timetemp > chinese_date {
        chinese_date = ChineseDate::new(date.year + 1, 1, 1).lunar_to_solar();
    }
    (chinese_date - timetemp) / (24 * 60 * 60)
}
