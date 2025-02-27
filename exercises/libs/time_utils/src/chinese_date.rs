use crate::date::Date;
pub const BASE_YEAR: u32 = 1900;
///! copy from hutool
// 这里计算中国新年
/// 记录了从1900年到2099年所有的农历数据
/// 农历表示：
/// * 1. 表示当年有无闰年，有的话，为闰月的月份，没有的话，为0。
/// * 2-4.为除了闰月外的正常月份是大月还是小月，1为30天，0为29天。
/// * 5. 表示闰月是大月还是小月，仅当存在闰月的情况下有意义。
pub const LUNAR_CODE: [u32; 200] = [
    0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, 0x16554, 0x056a0, 0x09ad0,
    0x055d2, //1900-1909
    0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540, 0x0d6a0, 0x0ada2, 0x095b0,
    0x14977, //1910-1919
    0x04970, 0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54, 0x02b60, 0x09570, 0x052f2,
    0x04970, //1920-1929
    0x06566, 0x0d4a0, 0x0ea50, 0x16a95, 0x05ad0, 0x02b60, 0x186e3, 0x092e0, 0x1c8d7,
    0x0c950, //1930-1939
    0x0d4a0, 0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0, 0x092d0, 0x0d2b2, 0x0a950,
    0x0b557, //1940-1949
    0x06ca0, 0x0b550, 0x15355, 0x04da0, 0x0a5b0, 0x14573, 0x052b0, 0x0a9a8, 0x0e950,
    0x06aa0, //1950-1959
    0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260, 0x0f263, 0x0d950, 0x05b57,
    0x056a0, //1960-1969
    0x096d0, 0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250, 0x0d558, 0x0b540, 0x0b6a0,
    0x195a6, //1970-1979
    0x095b0, 0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50, 0x06d40, 0x0af46, 0x0ab60,
    0x09570, //1980-1989
    0x04af5, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58, 0x05ac0, 0x0ab60, 0x096d5,
    0x092e0, //1990-1999
    0x0c960, 0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0, 0x0abb7, 0x025d0, 0x092d0,
    0x0cab5, //2000-2009
    0x0a950, 0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0, 0x0a5b0, 0x15176, 0x052b0,
    0x0a930, //2010-2019
    0x07954, 0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6, 0x0a4e0, 0x0d260, 0x0ea65,
    0x0d530, //2020-2029
    0x05aa0, 0x076a3, 0x096d0, 0x04afb, 0x04ad0, 0x0a4d0, 0x1d0b6, 0x0d250, 0x0d520,
    0x0dd45, //2030-2039
    0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0aa50, 0x1b255, 0x06d20,
    0x0ada0, //2040-2049
    0x14b63, 0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6, 0x0ea50, 0x06b20, 0x1a6c4,
    0x0aae0, //2050-2059
    0x092e0, 0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, 0x05d55, 0x056a0, 0x0a6d0,
    0x055d4, //2060-2069
    0x052d0, 0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, 0x055a0, 0x0aba4, 0x0a5b0,
    0x052b0, //2070-2079
    0x0b273, 0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x14b55, 0x04b60, 0x0a570, 0x054e4,
    0x0d160, //2080-2089
    0x0e968, 0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0, 0x0a9d4, 0x0a2d0, 0x0d150,
    0x0f252, //2090-2099
];
fn get_code(chinese_year: u32) -> u32 {
    LUNAR_CODE[(chinese_year - BASE_YEAR) as usize]
}
// 返回该月有多少天
fn month_days(chinese_year: u32, chinese_month: u32) -> u32 {
    if (get_code(chinese_year) & (0x1000 >> chinese_month)) == 0 {
        29
    } else {
        30
    }
}
// 返回这一年有多少天
fn year_days(chinese_year: u32) -> u32 {
    let mut sum = 348; // 全为小月的基础数字 12 * 29 = 348；
    let mut i = 0x8000; // 为什么取0x8000 是因为只要检测中间的11个数字即可
    while i > 0x8 {
        if (get_code(chinese_year) & i) != 0 {
            sum = sum + 1;
        }
        i = i >> 1;
    }
    // 最后再加上闰月的大小月，如果有闰月，是多出来一个月的
    sum + leap_days(chinese_year)
}
// 返回该年闰月的天数
fn leap_days(chinese_year: u32) -> u32 {
    if leap_month(chinese_year) != 0 {
        if (get_code(chinese_year) & 0x10000) != 0 {
            return 30;
        } else {
            return 29;
        }
    }
    0
}
fn leap_month(chinese_year: u32) -> u32 {
    get_code(chinese_year) & 0xf
}
pub type ChineseDate = Date;
impl ChineseDate {
    // 该方法copy借鉴的java的hutool工具包
    // 返回的是时间戳
    pub fn lunar_to_solar(&mut self) -> u32 {
        // 第一步获取这个月有多少天
        let day = month_days(self.year, self.month);
        // 计算农历的时间差
        let mut offset = 0;
        for i in BASE_YEAR..self.year {
            offset += year_days(i);
        }
        // 处理今年有多少天
        let mut leap: u32;
        let mut is_add = false;
        for i in 1..self.month {
            leap = leap_month(self.year);
            if !is_add {
                if leap <= i && leap > 0 {
                    offset += leap_days(self.year);
                    is_add = true;
                }
            }
            offset += month_days(self.year, i);
        }
        // 转换闰月农历，需要补充该年闰月的前一个月时差
        if (get_code(self.year + 1) & 0xf) == 1 {
            offset += day;
        }
        // 获取阳历的时间戳
        ((offset + self.day - 31) * 86400u32) - 2203804800u32
    }
}
