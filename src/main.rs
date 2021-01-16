use std::io;
use std::char;

fn main() {
    println!("Booting");
    let rounds: u32 = read_number("rounds");

    println!("Rounds: {}", rounds);
    for _ in 0..rounds {
        unsafe {
            let mut input_number = String::new();
            println!("Please insert a number (input string)");
            io::stdin().read_line(&mut input_number).expect("Could not parse string");

            if input_number.len() > 1_000_000 {
                panic!("too many digits");
            }
            
            let mut palindrome: String = input_number.trim().to_string();

            for (_idx, c) in palindrome.chars().enumerate() {
                assert!(c.is_digit(10));
            }

            while !check_palindrome(&palindrome) {
                let size: usize = palindrome.len();
                for i in 0..(size / 2) {
                    let right_digit_index: usize = size - 1 - i;
                    let left_ascii: u32 = from_ascii(palindrome.chars().nth(i).unwrap());
                    let right_digit: u32 = from_ascii(palindrome.chars().nth(right_digit_index).unwrap());

                    if left_ascii == right_digit {
                        continue; // nothing to do
                    } else if left_ascii > right_digit {
                        // 501 => 505
                        palindrome.replace_range(right_digit_index..right_digit_index + 1, &(left_ascii).to_string());
                    } else{
                        // 105 => 106
                        increment_digit(&mut palindrome, right_digit, right_digit_index);
                    }
                }
            }
            println!("Found palindrome: {}", palindrome);
        }
    }
}

fn read_number(reason: &str) -> u32 {
    let mut input_number = String::new();
    println!("Please insert a number ({})", reason);
    io::stdin().read_line(&mut input_number).expect("Could not parse string");

    let input_number: u32 = match input_number.trim().parse() {
        Ok(number) => number,
        Err(_) => panic!(":-(")
    };

    return input_number
}

fn check_palindrome(str_number: &String) -> bool {
    let mut is_equal: bool = true;
    let digits: Vec<char> = str_number.chars().collect();

    for i in 0..(digits.len() / 2) {
        is_equal = is_equal && digits[i] == digits[digits.len() - 1 - i];
    }
    return is_equal;
}

unsafe fn increment_digit(palindrome: &mut String, digit: u32, position: usize) {
    if digit < 9 {
        // 1234 => 1235
        palindrome.replace_range(position..position + 1, &(digit + 1).to_string());
    }else{
        // 1299 => 1300
        assert!(position > 0);
        let neighboring_digit: u32 = from_ascii(palindrome.chars().nth(position - 1).unwrap());
        palindrome.replace_range(position..position + 1, "0");

        increment_digit(palindrome, neighboring_digit, position - 1);
    }
}

fn from_ascii(ascii_char: char) -> u32 {
    let number: u32 = ascii_char as u32;
    return number - 48; // 49 is 1 in ASCII
}