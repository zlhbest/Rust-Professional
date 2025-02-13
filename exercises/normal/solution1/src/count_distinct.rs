pub fn new_count_distinct(input_str: &str) -> usize {
    let mut bit_map = vec![false; 300usize];
    let mut result = 0;
    for item in input_str.split(",") {
        let sum = item.chars().map(|char_item| char_item as usize).sum();
        let hash_value = hash(sum, bit_map.len());
        if !bit_map[hash_value] {
            result += 1;
            bit_map[hash_value] = true;
        }
    }
    result
}
fn hash(char_number: usize, max_length: usize) -> usize {
    char_number % max_length
}
