use std::{ops::Sub, u32};

use crate::utils::{days_in_month, days_in_year, zeller};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}
impl From<&str> for Date {
    // str的形式是2025-01-01
    fn from(value: &str) -> Self {
        let value_str_arr: Vec<&str> = value.split("-").collect();
        Self {
            year: value_str_arr[0].parse().expect("year value parse error"),
            month: value_str_arr[1].parse().expect("month value parse error"),
            day: value_str_arr[2].parse().expect("day value parse error"),
        }
    }
}
impl Sub for Date {
    type Output = i32;
    fn sub(self, rhs: Self) -> Self::Output {
        let timetemp1 = self.date_to_timetemp() as i32;
        let timetemp2 = rhs.date_to_timetemp() as i32;
        (timetemp1 - timetemp2) / (24 * 60 * 60) - 1
    }
}
impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.partial_cmp(&other.year) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.month.partial_cmp(&other.month) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.day.partial_cmp(&other.day)
    }
}
// 常规函数
impl Date {
    pub fn new(year: u32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
    /// 转时间戳
    pub fn date_to_timetemp(&self) -> u32 {
        let mut total_days = 0;
        for y in 1970..self.year {
            total_days += days_in_year(y);
        }
        for m in 1..self.month {
            total_days += days_in_month(self.year, m);
        }
        total_days += self.day - 1;
        total_days as u32 * 24 * 3600
    }
    /// 从时间戳到日期
    pub fn timestamp_to_date(timestamp: u32) -> Date {
        let seconds_per_day = 60 * 60 * 24;
        let mut days = timestamp / seconds_per_day;
        let mut year = 1970;

        // 逐年计算
        loop {
            let days_in_year = days_in_year(year);
            if days < days_in_year {
                break;
            }
            days -= days_in_year;
            year += 1;
        }

        let mut month = 1;
        // 逐月计算
        loop {
            let days_in_current_month = days_in_month(year, month);
            if days < days_in_current_month {
                break;
            }
            days -= days_in_current_month;
            month += 1;
        }

        let day = days + 1;
        Date { year, month, day }
    }
    /// 获取该日期到今年所有的天数
    pub fn days_in_year(&self) -> u32 {
        let mut all_days = 0;
        for month in 1..self.month {
            all_days += days_in_month(self.year, month);
        }
        all_days += self.day;
        all_days
    }
    /// 获取该日期所在周的周数
    pub fn day_of_week(&self) -> u32 {
        zeller(self.year, self.month, self.day)
    }
    /// 获取该周是第几周
    pub fn week_of_year(&self, days_in_year: u32) -> u32 {
        let first_date = Date::new(self.year, 1, 1);
        let first_week_days = 7 - first_date.day_of_week() + 1;
        let end_date = Date::new(self.year, 12, 31);
        let last_week_day = end_date.day_of_week();
        let remain_days;
        if days_in_year < first_week_days {
            return 1;
        } else {
            remain_days = days_in_year - first_week_days;
        }
        let mut remain_weeks = remain_days.div_ceil(7);

        if first_date.day_of_week() <= 4 {
            remain_weeks += 1;
        }
        if self.month == 12 && self.day >= 31 - last_week_day {
            if last_week_day >= 4 {
                remain_weeks += 1;
            } else {
                return 1;
            }
        }
        remain_weeks
    }
}
