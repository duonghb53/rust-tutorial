use std::time::Instant;

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
        //println!("low: {}, high: {}", low, high);

        while low <= high {
            let middle = (low + high) / 2;
            //println!("middle: {}", middle);
            if Solution::is_common_prefix(&strs, middle) {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }
        let len = (low + high) / 2;
        //println!("low: {}, high: {}, len: {}", low, high, len);
        strs[0][0..len].to_string()
    }

    pub fn is_common_prefix(strs: &Vec<String>, len: usize) -> bool {
        let str1 = &strs[0][0..len];
        //println!("str1: {}", str1);

        for i in 1..strs.len() {
            if str1 != &strs[i][0..str1.len()] {
                return false;
            }
        }
        true 
    }
}

fn main() {
    let now = Instant::now();
	let t1 = now.elapsed();

	println!("Generating testcase...");
	let test_case = gen_test_case();
	let test_case_clone1 = test_case.clone();

	let test_case_0 = vec![
		"acc".to_string(),
		"aaa".to_string(),
		"aaba".to_string(),
	];

	let mut test_case_2 = test_case_0.clone();
	test_case_2.append(&mut test_case.clone());

	let test_case_2_clone = test_case_2.clone();

	let t2 = now.elapsed();
	println!("Testcase generated in {:.2?}", t2 - t1);


	let lcp = Solution::longest_common_prefix(test_case_0);
	println!("\nlcp=`{}` len={}", lcp, lcp.len());
	
	let t3 = now.elapsed();
	println!("Testcase 0 complete in {:.2?}", t3-t2);


	// Test case 1
	let lcp = Solution::longest_common_prefix(test_case);
	println!("\nlcp=`{}` len={}", lcp, lcp.len());

	let t4 = now.elapsed();
	println!("Testcase 1 complete in {:.2?}", t4-t3);


	// Test case 2
	let lcp = Solution::longest_common_prefix(test_case_2);
	println!("\nlcp=`{}` len={}", lcp, lcp.len());

	let t5 = now.elapsed();
	println!("Testcase 2 complete in {:.2?}", t5-t4);
}

fn gen_test_case() -> Vec<String> {
	// 1GB = 100,000 string * 10,000 length
	vec!["a".repeat(10000); 100000]
}