use std::vec;
const TIMESTEMP_VALUE: u32 = 24 * 60 * 60;
use crate::{chinese_date::ChineseDate, date::Date};
use lazy_static::lazy_static;
lazy_static! {
    static ref HOLIDAY_LIST: Vec<Holiday> = {
        let mut vec = Vec::new();
        // 春节
        vec.push(Holiday::new(1, 1, 7, 0,true));
        // 元旦
        vec.push(Holiday::new(1, 1, 1, 1,false));
        // 五一劳动节
        vec.push(Holiday::new(5, 1, 5, 0,false));
        // 后面需要随时加节日
        vec
    };
}
/// 目前所有的假期
#[derive(Debug)]
struct Holiday {
    // 休息月份
    start_month: u32,
    // 休息开始
    start_day: u32,
    // 是否延期到下一年
    add_year: u32,
    days: u32,
    is_chinese_holiday: bool,
}
impl Holiday {
    fn new(
        start_month: u32,
        start_day: u32,
        days: u32,
        add_year: u32,
        is_chinese_holiday: bool,
    ) -> Self {
        Self {
            start_month,
            start_day,
            add_year,
            days,
            is_chinese_holiday,
        }
    }
    fn to_timestemp(&self, year: u32) -> Vec<u32> {
        let start_timestemp = if self.is_chinese_holiday {
            ChineseDate::new(year, self.start_month, self.start_day).lunar_to_solar()
        } else {
            Date::new(year, self.start_month, self.start_day).date_to_timetemp()
        };
        let end_timestemp = start_timestemp + (self.days * TIMESTEMP_VALUE);
        vec![start_timestemp, end_timestemp]
    }
}
// 这里面存折所有的节假日
fn get_holiday(year: u32) -> Vec<Vec<u32>> {
    let mut vec = Vec::new();
    for holiday in HOLIDAY_LIST.iter() {
        vec.push(holiday.to_timestemp(year + holiday.add_year));
    }
    vec
}
pub fn days_to_trading(date: &Date) -> u32 {
    // 先看节假日
    let timestemp = date.date_to_timetemp() + TIMESTEMP_VALUE;
    let holiday_vec = get_holiday(date.year);
    for item in holiday_vec {
        if timestemp >= item[0] && timestemp <= item[1] {
            return (item[1] - timestemp) / TIMESTEMP_VALUE;
        }
    }
    match date.day_of_week() {
        7 => 0, // 周日
        5 => 2, // 周五，距离下周一开盘 2 天
        6 => 1, // 周六，距离周一开盘 2 天
        _ => 0, // 周一到周四
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn days_to_trading_test() {
        let date = "2025-12-31".into();
        let days = super::days_to_trading(&date);
        assert_eq!(days, 1);
    }
}
