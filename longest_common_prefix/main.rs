struct Solution;

impl Solution {

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        } 

        let min_len: usize = strs.iter().fold(&strs[0], |acc, item| {
            if item.len() < acc.len() {
                item
            } else {
                acc
            }}).len();

        let mut low = 1;
        let mut high = min_len;
        println!("low: {}, high: {}", low, high);

        while low <= high {
            let middle = (low + high) / 2;
            println!("middle: {}", middle);
            if Solution::is_common_prefix(&strs, middle) {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }
        let len = (low + high) / 2;
        println!("low: {}, high: {}, len: {}", low, high, len);
        strs[0][0..len].to_string()
    }

    pub fn is_common_prefix(strs: &Vec<String>, len: usize) -> bool {
        let str1 = &strs[0][0..len];
        println!("str1: {}", str1);

        for i in 1..strs.len() {
            if str1 != &strs[i][0..str1.len()] {
                return false;
            }
        }
        true 
    }
}

fn main() {
    let v = vec![String::from("hello"), String::from("hueueueeyeyryerye")]; 
    let test = Solution::longest_common_prefix(v);
    println!("Prefix: {}", test);
}