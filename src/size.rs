use std::collections::HashMap;

use rand::{thread_rng, Rng};
use regex::{Match, Regex};


/// Turns the input string into an integer of type usize.  
/// The input can be any decimal number with or without a unit, whose integer and fraction parts are separated by either a dot or a comma.
/// Negative numbers are not allowed.
/// The unit of the requested size can either be `k` (*10³), `m` (*10⁶) or `g` (*10⁹) and is case insensitive.
/// 
/// Examples of valid strings are: "123", "123k", "123,4m", "123.4567G", ".456m".  
/// 
/// When a unit is provided, the unspecified digits will be filled randomly with consideration of the deviation factor.
/// 
/// **Example**:  
/// The arguments of the function are input = "123.456G" and deviation = 0.2.
/// Converted to bytes, the number provided by input is 123456000. The last three zeroes are unspecified and will be randomized.
/// How large the random range should be is determined by the number of unspecified digits and the deviation factor.
/// The random number range goes from `-(reference_value * deviation)` to `+(reference_value * deviation)`.
/// The reference value is the result of `10^(number of unspecified digits)`.
/// 
/// In this example, the reference_value equals 1000, since there are three unspecified digits.
/// Combined with the deviation factor, that results in a range from -200 to 200.
/// 
/// That means, the resulting size will be between 123455800 bytes and 123456200 bytes.
/// 
/// If you don't want to randomize the lase digits, you can either enter a deviation factor of 0 or specify all digits of your number.
/// 
pub fn from(input: String, deviation: f64) -> Option<usize> {
    let (left_of_dot, right_of_dot, unit_opt) = capture_parts(input)?;

    let (int_part_str, fraction_part_str) = convert_to_num_parts(left_of_dot, right_of_dot);

    let factor = get_factor(unit_opt);
    let offset = factor.to_string().len() - 1;

    let input_number = parse_to_usize_with_offset(&int_part_str, &fraction_part_str, offset)?;
    if input_number == 0 {
        return None;
    }

    let fraction_part_str_len = fraction_part_str.unwrap_or(String::new()).len();
    let adjusted_factor = factor.checked_div(ten_to_the_power_of(fraction_part_str_len)?)?;

    let random_number = randomize_around(adjusted_factor, deviation);

    if random_number >= 0 {
        input_number.checked_add(random_number as usize)
    } else {
        input_number.checked_sub(random_number.abs() as usize)
    }
}

/// Tests whether the input string matches the file size regex and returns the three captures that can be used to calculate the size.  
/// Any string that matches the expression `^((\d*)\.)?(\d+)([kmg])?$` is allowed.
/// 
/// Examples of allowed strings are:  
/// - "123"  
/// - "123k"  
/// - "123.45m"  
/// - "123.4567g"  
/// - ".456m"
/// 
/// The **first capture** represents the **number left of the dot**.  
/// The **second capture** contains the **number to its right**.  
/// The **third capture** returns the **unit** of the number.  
/// 
/// **Note**: If there is no dot in the string, the number in the string will be returned in the *second capture* (number to the right of the dot).
fn capture_parts<'a> (input: String) -> Option<(Option<String>, Option<String>, Option<String>)> {
    /* Capture groups in the regex string: 
     * 0: 123.456m  (whole string -> unused)
     * 1: 123.      (with dot -> unused)
     * 2: 123       (without dot -> left of dot)
     * 3: 456       (right of dot)
     * 4: m         (unit)
     */
    let input = input.replace(",", ".").to_lowercase();
    let regex = Regex::new(r"^((\d*)\.)?(\d+)([kmg])?$").unwrap();
    let captures_opt = regex.captures(&input);
    if let Some(captures) = captures_opt {
        Some((
            map_to_string(captures.get(2)), 
            map_to_string(captures.get(3)), 
            map_to_string(captures.get(4)),
        ))
    } else {
        None
    }
}

/// Converts the regex captures into the integer part and the fraction part of a number.
fn convert_to_num_parts<'a>(left_capture: Option<String>, right_capture: Option<String>) -> (String, Option<String>) {
    let mut number_str = right_capture.unwrap();
    let mut decimal_str_opt = None;
    if let Some(left_of_dot) = left_capture {
        decimal_str_opt = Some(number_str);
        number_str = left_of_dot;
    }
    (number_str, decimal_str_opt)
}

/// Turns the provided unit into a factor:
/// - `k` becomes 1000
/// - `m` becomes 1000000
/// - `g` becomes 1000000000
/// 
/// If no unit is provided, the factor is 1.
fn get_factor(capture_opt: Option<String>) -> usize {
    let mut factor_map = HashMap::with_capacity(3);
    factor_map.insert(String::from("k"), 1000);
    factor_map.insert(String::from("m"), 1000000);
    factor_map.insert(String::from("g"), 1000000000);
    
    match capture_opt {
        Some(unit) => factor_map[&unit],
        None => 1,
    }
}


/// Turns the given decimal number into a usize.
/// The offset references the right end of the number_str and pads the resulting number with zeroes on the right side.  
/// This function checks whether the content of `decimal_str_opt` (if it has a value) is not longer than the offset and returns None in that case.
/// It also makes sure that the strings can be converted into usize.  
/// 
/// Examples:  
/// ```
/// parse_to_usize_with_offset("123", Some("456"), 5) == Some(12345600)
/// parse_to_usize_with_offset("123", None, 4) == Some(1230000)
/// parse_to_usize_with_offset("123", Some("456"), 2) => None
/// ```
fn parse_to_usize_with_offset(int_part_str: &String, fraction_part_str_opt: &Option<String>, offset: usize) -> Option<usize> {
    match fraction_part_str_opt {
        Some(fraction_part_str) => {
            let adjusted_offset = offset.checked_sub(fraction_part_str.len())?;
            let combined_number = format!("{}{}", int_part_str, fraction_part_str).parse::<usize>().ok()?;
            combined_number.checked_mul(ten_to_the_power_of(adjusted_offset)?)
        },
        None => {
            int_part_str.parse::<usize>().ok()?.checked_mul(ten_to_the_power_of(offset)?)
        },
    } 
}

/// Returns a random value between `-value*deviation` and `value*deviation`.
fn randomize_around(value: usize, deviation: f64) -> isize {
    let difference = (value as f64 * deviation) as isize;
    return thread_rng().gen_range(-difference..=difference)
}

fn map_to_string(capture: Option<Match>) -> Option<String> {
    capture.map(|m| m.as_str().to_string())
}

fn ten_to_the_power_of(exp: usize) -> Option<usize> {
    (10 as usize).checked_pow(exp as u32)
}


#[cfg(test)]
mod tests {
    use crate::size::*;

    #[test]
    fn size_from_success() {
        assert_eq!(from("123.456".to_string(), 0.0), None);
        assert_eq!(from("123,456".to_string(), 0.0), None);
        assert_eq!(from("123456".to_string(), 0.0), Some(123456));

        assert_eq!(from("123.456k".to_string(), 0.0), Some(123456));
        assert_eq!(from("123.456m".to_string(), 0.0), Some(123456000));
        assert_eq!(from("123.456g".to_string(), 0.0), Some(123456000000));
        assert_eq!(from("123,456k".to_string(), 0.0), Some(123456));
        assert_eq!(from("123,456m".to_string(), 0.0), Some(123456000));
        assert_eq!(from("123,456g".to_string(), 0.0), Some(123456000000));
        assert_eq!(from("123.456K".to_string(), 0.0), Some(123456));
        assert_eq!(from("123.456M".to_string(), 0.0), Some(123456000));
        assert_eq!(from("123.456G".to_string(), 0.0), Some(123456000000));
        assert_eq!(from("123,456K".to_string(), 0.0), Some(123456));
        assert_eq!(from("123,456M".to_string(), 0.0), Some(123456000));
        assert_eq!(from("123,456G".to_string(), 0.0), Some(123456000000));
        
        assert_eq!(from(".456m".to_string(), 0.0), Some(456000));
        assert_eq!(from("123.m".to_string(), 0.0), None);
        assert_eq!(from("123.4567k".to_string(), 0.0), None);

        let test_result_1 = from("123.456m".to_string(), 0.2).unwrap();
        assert!(123455800 < test_result_1 && test_result_1 < 123456200)
    }

    #[test]
    fn parse_to_usize_with_offset_success() {
        assert_eq!(parse_to_usize_with_offset(&"123".to_string(), &Some("456".to_string()), 5), Some(12345600));
        assert_eq!(parse_to_usize_with_offset(&"123".to_string(), &None, 4), Some(1230000));
        assert_eq!(parse_to_usize_with_offset(&"123".to_string(), &Some("456".to_string()), 3), Some(123456));
        assert_eq!(parse_to_usize_with_offset(&"123".to_string(), &None, 0), Some(123));
        assert_eq!(parse_to_usize_with_offset(&"123".to_string(), &Some("456".to_string()), 2), None);
        assert_eq!(parse_to_usize_with_offset(&"abc".to_string(), &Some("456".to_string()), 4), None);
        assert_eq!(parse_to_usize_with_offset(&"123".to_string(), &Some("def".to_string()), 4), None);
    }
}