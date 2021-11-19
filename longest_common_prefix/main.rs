struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = "";
        
        for elem in strs.iter() {
            println!("elem: {}", elem);
            //println!("elem: {}",strs.strip_prefix(&strs[0]), Some(&elem));
            println!("Some: {:?}", strs.strip_prefix(elem.as_bytes()));
        }
        
        prefix.to_string()
    }
}
fn main() {
    let v = vec![String::from("hello"), String::from("world")]; 
    let test = Solution::longest_common_prefix(v);
    println!("Prefix: {}", test);
}