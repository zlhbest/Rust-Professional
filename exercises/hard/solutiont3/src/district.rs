use std::{collections::HashMap, env, error::Error, fs::File};

use serde::{de::Visitor, Deserialize};
/// 查看该数据有哪些城市群
pub fn process(input: Vec<CityNode>) -> usize {
    let mut city_nodes_list: Vec<Vec<String>> = Vec::new();
    for mut city_node in input {
        if city_nodes_list.is_empty() {
            city_nodes_list.push(city_node.group.clone());
        } else {
            let mut is_new: bool = true;
            for item in &mut city_nodes_list {
                let mut should_append = false;
                // 看这个节点里面的节点是否在这个组里面，在的话就加入，不在就算了
                for group_item in &city_node.group {
                    // 如果在这个群里面我就把所有的加入到这个item中
                    if item.contains(&group_item) {
                        should_append = true;
                        break;
                    }
                }
                // 将所有的加入
                if should_append {
                    item.append(&mut city_node.group);
                    is_new = false;
                }
            }
            if is_new {
                city_nodes_list.push(city_node.group.clone());
            }
        }
    }
    return city_nodes_list.len();
}

pub fn read_json_to_obj() -> Result<HashMap<u32, CityNodeList>, Box<dyn Error>> {
    let path = env::current_dir()?;
    // 读数据
    let json_file = File::open(path.join("district.json"))?;
    // 解析数据
    let json_map: HashMap<u32, CityNodeList> = serde_json::from_reader(json_file)?;
    Ok(json_map)
}

#[derive(Debug, Clone, Deserialize)]
pub struct CityNode {
    pub name: String,
    pub group: Vec<String>,
}

impl PartialEq for CityNode {
    fn eq(&self, other: &Self) -> bool {
        // 只看名称
        self.name == other.name
    }
}
impl CityNode {
    pub fn new(name: String, nodes: Vec<String>) -> Self {
        let mut group = nodes;
        group.push(name.clone());
        CityNode { name, group }
    }
}
#[derive(Debug)]
pub struct CityNodeList {
    pub list: Vec<CityNode>,
}
impl CityNodeList {
    fn new(list: Vec<CityNode>) -> Self {
        CityNodeList { list }
    }
}
impl<'de> Deserialize<'de> for CityNodeList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CityNodeListVisitor;
        impl<'de> Visitor<'de> for CityNodeListVisitor {
            type Value = CityNodeList;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("json to CityNodeList")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut city_list = Vec::new();
                while let Some((key, value)) = map.next_entry()? {
                    city_list.push(CityNode::new(key, value));
                }
                Ok(CityNodeList::new(city_list))
            }
        }
        deserializer.deserialize_any(CityNodeListVisitor)
    }
}

pub fn count_provinces() -> String {
    // 读表
    match read_json_to_obj() {
        Ok(item) => {
            let mut result: Vec<String> = Vec::with_capacity(5);
            for index in 1..=(item.len() as u32) {
                let count = process(item.get(&index).unwrap().list.clone());
                result.push(count.to_string());
            }
            return result.join(",");
        }
        Err(error) => {
            println!("error:{}", error);
            return String::from("");
        }
    }
}
