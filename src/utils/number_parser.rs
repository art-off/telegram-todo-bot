use std::collections::HashSet;
use regex::{Captures, Regex};

pub struct NumberParser {
    origin_string: String,
    modes: Vec<NumberParserMode>
}

#[derive(PartialEq)]
pub enum NumberParserMode {
    SingleNumbers,
    NumberRanges,
}

impl NumberParser {
    pub fn new(origin_string: String, modes: Vec<NumberParserMode>) -> Self {
        Self { origin_string, modes }
    }

    pub fn parse(&self) -> Vec<i32> {
        let mut parsed_string = self.origin_string.clone();
        let mut result_nums: Vec<i32> = vec![];

        // Почему-то это работает как говно (и крашит)
        //
        // if self.modes.contains(&NumberParserMode::NumberRanges) {
        //     let num_ranges_re = Regex::new(r"(\d+)\s*-\s*(\d+)").unwrap();
        //     let tmp_string = parsed_string.clone();
        //     for cap in num_ranges_re.captures_iter(&tmp_string) {
        //         if let (Some(start), Some(end)) = (cap.get(0), cap.get(1)) {
        //             let start_num: i32 = start.as_str().parse().unwrap();
        //             let end_num: i32 = end.as_str().parse().unwrap();
        //
        //             (start_num..end_num).for_each(|x| result_nums.push(x));
        //             parsed_string = parsed_string.replace(start.as_str(), "");
        //         }
        //     }
        // }

        if self.modes.contains(&NumberParserMode::SingleNumbers) {
            let nums_re = Regex::new(r"(\d+)").unwrap();
            let tmp_string = parsed_string.clone();
            for cap in nums_re.captures_iter(&tmp_string) {
                if let Some(num_match) = cap.get(0) {
                    let num: i32 = num_match.as_str().parse().unwrap();
                    result_nums.push(num);
                    parsed_string = parsed_string.replace(num_match.as_str(), "");
                }
            }
        }

        let hash_set: HashSet<i32> = result_nums.into_iter().collect();
        let result_vec: Vec<i32> = hash_set.into_iter().collect();

        result_vec
    }
}