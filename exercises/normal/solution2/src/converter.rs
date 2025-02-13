use ::num::pow;
struct BaseEntry {
    // 初始化就计算出十进制数字
    ten_base_value: u32,
    // 数字
    value: Option<Vec<u32>>,
    // 进制
    base: u32,
}
impl BaseEntry {
    // 解析
    fn new(num_str: &str) -> Self {
        // 解析这个变成实体
        let mut result = BaseEntry {
            ten_base_value: 0,
            value: Option::None,
            base: 0,
        };
        let item: Vec<&str> = num_str.split("(").collect();

        result.value = Some(
            item[0]
                .chars()
                .map(|item| match item {
                    'a' => 10,
                    'b' => 11,
                    'c' => 12,
                    'd' => 13,
                    'e' => 14,
                    'f' => 15,
                    _ => String::from(item).parse::<u32>().unwrap(),
                })
                .collect(),
        );
        result.base = item[1].replace(")", "").parse().unwrap();
        // 转成十进制
        let temp = result.value.clone().unwrap();
        for (index, item) in temp.iter().rev().enumerate() {
            result.ten_base_value =
                result.ten_base_value + (*item as u32) * pow(result.base, index);
        }
        result
    }
    fn translate(self, to_base: u32) -> BaseEntry {
        let mut reuslt = BaseEntry {
            ten_base_value: 0,
            value: None,
            base: to_base,
        };
        // 转换
        let mut stack: Vec<u32> = Vec::new();
        let mut temp = self.ten_base_value;
        while temp > 0 {
            stack.push(temp % to_base);
            temp = temp / to_base;
        }
        reuslt.value = Some(stack);
        reuslt
    }

    fn to_base(self) -> String {
        let mut stack = self.value.unwrap();
        let mut value = String::from("");
        while !stack.is_empty() {
            let item = stack.pop().unwrap();
            let item_char = match item {
                10 => String::from("a"),
                11 => String::from("b"),
                12 => String::from("c"),
                13 => String::from("d"),
                14 => String::from("e"),
                15 => String::from("f"),
                _ => item.to_string(),
            };
            value = value + &item_char;
        }
        format!("{}", value)
    }
}
pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // 将***(**)解析出来
    let num_entry = BaseEntry::new(num_str);
    // 然后解析出来进行转换
    let reuslt = num_entry.translate(to_base);
    reuslt.to_base()
}
