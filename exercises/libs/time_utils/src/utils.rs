pub fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => panic!("Invalid month"),
    }
}
pub fn days_in_year(year: u32) -> u32 {
    if is_leap_year(year) { 366 } else { 365 }
}
pub fn zeller(year: u32, month: u32, day: u32) -> u32 {
    let q = day;
    let (m, y) = if month < 3 {
        (month + 12, year - 1)
    } else {
        (month, year)
    };
    let weekday: u32 =
        (q + (26 * (m + 1) / 10) + (y % 100) + ((y % 100) / 4) + ((y / 100) / 4) + (5 * (y / 100)))
            % 7;
    match weekday {
        0 => 6,
        1 => 7,
        2 => 1,
        3 => 2,
        4 => 3,
        5 => 4,
        6 => 5,
        _ => 0,
    }
}

pub fn all_days_in_year(year: u32) -> u32 {
    let mut sum_days = 337;
    if is_leap_year(year) {
        sum_days += 29;
    } else {
        sum_days += 28;
    }
    sum_days
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
