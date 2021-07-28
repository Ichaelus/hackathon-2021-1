use std::char;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

fn main() -> Result<(), String> {
    let rounds = read_number()?;
    for _ in 0..rounds {
        let mut input_number = String::new();
        io::stdin()
            .read_line(&mut input_number)
            .map_err(|err| err.to_string())?;

        let num_digits = input_number.len();
        if num_digits > 1_000_000 {
            return Err(format!("Too many digits: {}!", num_digits));
        }

        let next_palindrome = next_palindrome(&input_number)?;
        println!("{}", &next_palindrome);
    }
    Ok(())
}

fn read_number() -> Result<usize, String> {
    let mut input_number = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .map_err(|err| err.to_string())?;
    let input_number: usize = input_number
        .trim()
        .parse()
        .map_err(|err: ParseIntError| err.to_string())?;
    Ok(input_number)
}

fn next_palindrome(palindrome: &str) -> Result<String, String> {
    let mut palindrome = get_char_vec_and_validate_digits(palindrome)?;
    let size = palindrome.len();

    // If the number is already a palindrome, always mutate it
    if check_palindrome(&palindrome) {
        let right_digit_index = size - 1;
        let increment_to_index = increment_digit_at_position(&mut palindrome, right_digit_index);
        if increment_to_index == 0 {
            return Ok(set_last_digit_value_to_first_digit_value(&mut palindrome));
        }
    }

    // Transform the digit vec into the next palindrome
    update_string_from_right_to_middle(&mut palindrome);
    update_middle_digits(&mut palindrome, size);

    Ok(palindrome.into_iter().collect())
}

fn get_char_vec_and_validate_digits(palindrome: &str) -> Result<Vec<char>, String> {
    let palindrome = palindrome
        .trim()
        .chars()
        .map(|ascii_char| {
            if !ascii_char.is_digit(10) {
                return Err(format!("Non-digit character found! {}", ascii_char));
            }
            Ok(ascii_char)
        })
        .collect::<Result<Vec<char>, String>>()?;
    Ok(palindrome)
}

fn update_string_from_right_to_middle(mut palindrome: &mut Vec<char>) {
    let size = palindrome.len();
    for index in 0..(size / 2) {
        let right_digit_index = size - 1 - index;
        let left_digit = get_digit_at_position_from_str(&palindrome, index);
        let right_digit = get_digit_at_position_from_str(&palindrome, right_digit_index);

        match left_digit.cmp(&right_digit) {
            Ordering::Greater => {
                set_position_to_digit(&mut palindrome, right_digit_index, left_digit);
            }
            Ordering::Less => {
                let incremented_to_index =
                    increment_digit_at_position(&mut palindrome, right_digit_index - 1);
                let maybe_changed_left_digit = get_digit_at_position_from_str(&palindrome, index);
                set_position_to_digit(&mut palindrome, right_digit_index, maybe_changed_left_digit);
                if incremented_to_index < (size / 2) {
                    update_right_after_increment_overflow_left(
                        &mut palindrome,
                        incremented_to_index,
                    );
                    return;
                }
            }
            _ => (),
        }
    }
}

fn update_middle_digits(palindrome: &mut Vec<char>, size: usize) {
    let middle_right_index = size / 2;
    let middle_left_index = (size - 1) / 2;
    let left_digit = get_digit_at_position_from_str(&palindrome, middle_left_index);
    let right_digit = get_digit_at_position_from_str(&palindrome, middle_right_index);
    if left_digit >= right_digit {
        set_position_to_digit(palindrome, middle_right_index, left_digit);
    } else {
        set_position_to_digit(palindrome, middle_left_index, left_digit + 1);
        set_position_to_digit(palindrome, middle_right_index, left_digit + 1);
    }
}

fn update_right_after_increment_overflow_left(
    palindrome: &mut Vec<char>,
    incremented_to_index: usize,
) {
    let palindrome_len = palindrome.len();
    for index in incremented_to_index..(palindrome_len / 2) {
        let right_index = palindrome_len - index - 1;
        let left_digit: u8 = get_digit_at_position_from_str(&palindrome, index);
        set_position_to_digit(palindrome, right_index, left_digit);
    }
}

fn set_last_digit_value_to_first_digit_value(palindrome: &mut Vec<char>) -> String {
    let last_index = palindrome.len() - 1;
    let digit = get_digit_at_position_from_str(palindrome, 0);
    set_position_to_digit(palindrome, last_index, digit);
    palindrome.iter().collect()
}

fn get_digit_at_position_from_str(palindrome: &[char], position: usize) -> u8 {
    let ascii_char = palindrome[position];
    ascii_char.to_digit(10).map(|digit| digit as u8).unwrap()
}

fn check_palindrome(str_number: &[char]) -> bool {
    let size = str_number.len();
    for index in 0..size {
        let right_digit_index = size - 1 - index;
        let left_digit = get_digit_at_position_from_str(&str_number, index);
        let right_digit = get_digit_at_position_from_str(&str_number, right_digit_index);
        if left_digit != right_digit {
            return false;
        }
    }
    true
}

fn set_position_to_digit(palindrome: &mut Vec<char>, position: usize, new_digit: u8) {
    palindrome[position] = char::from_digit(new_digit as u32, 10).unwrap();
}

fn increment_digit_at_position(palindrome: &mut Vec<char>, position: usize) -> usize {
    for index in (0..=position).rev() {
        let digit = get_digit_at_position_from_str(&palindrome, index);
        if digit < 9 {
            set_position_to_digit(palindrome, index, digit + 1);
            return index;
        } else {
            palindrome[index] = '0';
        }
    }
    palindrome[0] = '1';
    palindrome.insert(1, '0');
    0
}

#[cfg(test)]
mod test {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_increment_digit() {
        let mut x = String::from("123").chars().collect();
        increment_digit_at_position(&mut x, 1);
        assert_eq!(x.into_iter().collect::<String>(), String::from("133"));

        let mut x = String::from("199").chars().collect();
        increment_digit_at_position(&mut x, 2);
        assert_eq!(x.into_iter().collect::<String>(), String::from("200"));

        let mut x = String::from("9").chars().collect();
        increment_digit_at_position(&mut x, 0);
        assert_eq!(x.into_iter().collect::<String>(), String::from("10"));

        let mut x = String::from("99").chars().collect();
        increment_digit_at_position(&mut x, 1);
        assert_eq!(x.into_iter().collect::<String>(), String::from("100"));

        let mut x = String::from("100").chars().collect();
        increment_digit_at_position(&mut x, 2);
        assert_eq!(x.into_iter().collect::<String>(), String::from("101"));

        let mut x = String::from("98792798472").chars().collect();
        increment_digit_at_position(&mut x, 10);
        assert_eq!(
            x.into_iter().collect::<String>(),
            String::from("98792798473")
        );

        let mut x = String::from("999999999999999999999").chars().collect();
        increment_digit_at_position(&mut x, 20);
        assert_eq!(
            x.into_iter().collect::<String>(),
            String::from("1000000000000000000000")
        );
    }

    #[test]
    fn test_next_palindrome() -> Result<(), String> {
        let x = String::from("899998");
        assert_eq!(next_palindrome(&x)?, String::from("900009"));

        let x = String::from("1239400");
        assert_eq!(next_palindrome(&x)?, String::from("1240421"));

        let x = String::from("123456");
        assert_eq!(next_palindrome(&x)?, String::from("124421"));

        let x = String::from("99");
        assert_eq!(next_palindrome(&x)?, String::from("101"));

        let x = String::from("2133");
        assert_eq!(next_palindrome(&x)?, String::from("2222"));

        let x = String::from("1287361983619826");
        assert_eq!(next_palindrome(&x)?, String::from("1287361991637821"));

        let x = String::from("8080808080");
        assert_eq!(next_palindrome(&x)?, String::from("8080880808"));

        let x = String::from("808");
        assert_eq!(next_palindrome(&x)?, String::from("818"));

        let x = String::from("999");
        assert_eq!(next_palindrome(&x)?, String::from("1001"));

        let x = String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111112");
        assert_eq!(next_palindrome(&x)?, String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111122111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"));

        Ok(())
    }
}
