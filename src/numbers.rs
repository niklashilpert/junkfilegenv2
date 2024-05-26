use std::collections::HashMap;

use rand::{thread_rng, Rng};
use regex::{Match, Regex};

pub fn to_file_size(input: &str, random_variance: f64) -> Option<usize> {
    let input = input.replace(",", ".").to_lowercase();

    /*
     * Capture Groups: 123.456m
     * 1: 123.
     * 2: 123
     * 3: 456
     * 4: m
     */
    let format = Regex::new(r"^((\d*)\.)?(\d+)([kmg])?$").unwrap();
    let captures = format.captures(&input)?;

    let factor = get_factor(captures.get(4));
     
    let no_dot = captures.get(2).is_none();

    if no_dot {
        parse_without_dot(captures.get(3), factor, random_variance)
    } else {
        parse_with_dot(captures.get(2), captures.get(3), factor, random_variance)
    }
}

fn get_factor(capture_opt: Option<Match>) -> usize {
    let mut factor_map = HashMap::with_capacity(3);
    factor_map.insert(String::from("k"), 1000);
    factor_map.insert(String::from("m"), 1000000);
    factor_map.insert(String::from("g"), 1000000000);
    
    return match capture_opt {
        Some(unit) => factor_map[unit.as_str()],
        None => 1,
    }
}

fn randomize_around(value: usize, variance: f64) -> isize {
    let difference = (value as f64 * variance) as isize;
    println!("{}", difference);
    return thread_rng().gen_range(-difference..=difference)
}

fn parse_without_dot(number_capture: Option<Match>, factor: usize, random_variance: f64) -> Option<usize> {
    let num_str = number_capture.unwrap().as_str();
    let num = num_str.parse::<usize>().unwrap();
    let generated_number = randomize_around(factor, random_variance);
    println!("{}", generated_number);
    return Some(((num*factor) as isize + generated_number) as usize)
}

fn parse_with_dot(
    left_capture: Option<Match>, 
    right_capture: Option<Match>, 
    factor: usize, 
    random_variance: f64
) -> Option<usize> {
    let left_str = left_capture.unwrap().as_str();
    let left_num = left_str.parse::<usize>().unwrap();

    let right_str = right_capture.unwrap().as_str();
    let right_len = right_str.len();
    let right_num = right_str.parse::<usize>().unwrap();
    
    let factor_len = factor.to_string().len();

    if factor_len-1 < right_len {
        return None;
    }

    let spaces_to_fill = factor_len-1 - right_len;
    let existing_numbers_factor = (10 as usize).pow(spaces_to_fill as u32);
    
    let number_before_randomizing = left_num * factor + right_num * existing_numbers_factor;
    let generated_number = randomize_around(existing_numbers_factor, random_variance);

    println!("{}", generated_number);


    return Some((number_before_randomizing as isize + generated_number) as usize);
}
