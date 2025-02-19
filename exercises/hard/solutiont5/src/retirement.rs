// 这里的想法是使用一个设计模式来处理，使用策略模式，将退休策略与各种情况绑定。
use std::ops::AddAssign;
//退休年龄调整
// 男职工：从 2025 年 1 月 1 日起，按每 4 个月延迟 1 个月的节奏，用 15 年时间将法定退休年龄逐步延迟至 63 周岁. 原定是60岁退休
// 女职工：原法定退休年龄为 55 周岁的女职工，从 2025 年 1 月 1 日起，按每 4 个月延迟 1 个月的节奏，用 15 年时间将法定退休年龄逐步延迟至 58 周岁；
// 原法定退休年龄为 50 周岁的女职工，按每 2 个月延迟 1 个月的节奏，用 15 年时间将法定退休年龄逐步延迟至 55 周岁.
const BASE_RERIRE_YRER: usize = 2025; // 实际上是2025年一月一日
fn calculate_retire_age(birth: &Date, retire_date: &Date) -> f64 {
    let mut years_diff = retire_date.year - birth.year;
    let months_diff = retire_date.month as i32 - birth.month as i32;

    if months_diff < 0 {
        years_diff -= 1;
        let adjusted_months_diff = months_diff + 12;
        years_diff as f64 + adjusted_months_diff as f64 / 12.0
    } else {
        years_diff as f64 + months_diff as f64 / 12.0
    }
}
struct Date {
    year: usize,
    month: usize,
}
impl Date {
    fn new(time: &str) -> Date {
        let time_vec = time
            .split("-")
            .map(|item| item.parse().unwrap())
            .collect::<Vec<usize>>();
        Date {
            year: time_vec[0],
            month: time_vec[1],
        }
    }
}
impl AddAssign for Date {
    // 这里进行加等的操作
    fn add_assign(&mut self, rhs: Self) {
        let mut new_month = self.month + rhs.month;
        let mut new_year = self.year + rhs.year;
        while new_month > 12 {
            new_month -= 12;
            new_year += 1;
        }
        self.month = new_month;
        self.year = new_year;
    }
}
struct RetireInfo {
    retire_date: Date,
    retire_age: f64,
    delay_monthes: usize,
}
impl RetireInfo {
    fn format(&self) -> String {
        fn format_number(num: f64) -> String {
            let num_str = format!("{:.2}", num);
            if num_str.ends_with(".00") {
                num_str[..num_str.len() - 3].to_string()
            } else {
                num_str
            }
        }
        format!(
            "{}-{:02},{},{}",
            self.retire_date.year,
            self.retire_date.month,
            format_number(self.retire_age),
            self.delay_monthes
        )
    }
}
// 使用策略模式处
trait RetireStrategy {
    // 返回一个明确的退休日期
    fn calculater(&self, birth_date: Date) -> RetireInfo;
}
struct ErrorStrategy;
impl RetireStrategy for ErrorStrategy {
    fn calculater(&self, _birth_date: Date) -> RetireInfo {
        RetireInfo {
            retire_date: Date { year: 0, month: 0 },
            retire_age: 0.0,
            delay_monthes: 0,
        }
    }
}
struct MaleEmployee;
impl RetireStrategy for MaleEmployee {
    fn calculater(&self, birth_date: Date) -> RetireInfo {
        calculater_base(birth_date, 60, 4, 36)
    }
}
struct FemaleEmloyee55;
impl RetireStrategy for FemaleEmloyee55 {
    fn calculater(&self, birth_date: Date) -> RetireInfo {
        calculater_base(birth_date, 55, 4, 36)
    }
}

struct FemaleEmployee50;
impl RetireStrategy for FemaleEmployee50 {
    // 计算原法定退休年龄50周岁女职工的员工
    fn calculater(&self, birth_date: Date) -> RetireInfo {
        calculater_base(birth_date, 50, 2, 60)
    }
}
fn calculater_base(
    birth_date: Date,
    retrie_year: usize,
    month: usize,
    max_month: usize,
) -> RetireInfo {
    // 第一步计算出生日期到退休
    let retire_year = birth_date.year + retrie_year;
    if retire_year < BASE_RERIRE_YRER {
        return RetireInfo {
            retire_date: Date {
                year: retire_year,
                month: birth_date.month,
            },
            retire_age: retrie_year as f64,
            delay_monthes: 0,
        };
    }
    // 拿退休时间和2025年做一个差，看有多少个月份
    let monthes = (retire_year - BASE_RERIRE_YRER) * 12 + birth_date.month;
    let mut retire_monthes = if monthes > month { monthes / month } else { 1 };
    retire_monthes = if retire_monthes > max_month {
        max_month
    } else {
        retire_monthes
    };
    let retire_month = birth_date.month;
    let mut retire_time = Date {
        year: retire_year,
        month: retire_month,
    };
    // 这里就是加这些月份了
    retire_time += Date {
        year: 0,
        month: retire_monthes,
    };
    let retire_age = calculate_retire_age(&birth_date, &retire_time);
    RetireInfo {
        retire_date: retire_time,
        retire_age,
        delay_monthes: retire_monthes,
    }
}

// 主函数
pub fn retire_time(time: &str, tp: &str) -> String {
    let strategy: Box<dyn RetireStrategy> = match tp {
        "男职工" => Box::new(MaleEmployee),
        "原法定退休年龄50周岁女职工" => Box::new(FemaleEmployee50),
        "原法定退休年龄55周岁女职工" => Box::new(FemaleEmloyee55),
        _ => Box::new(ErrorStrategy),
    };
    let retire_info = strategy.calculater(Date::new(time));
    retire_info.format()
}
