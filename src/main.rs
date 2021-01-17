use std::io;
use std::char;

fn main() {
    //println!("Booting");
    let rounds: u32 = read_number();

    //println!("Rounds: {}", rounds);
    for _ in 0..rounds {
        let mut input_number = String::new();
        //println!("Please insert a number (input string)");
        io::stdin().read_line(&mut input_number).expect("Could not parse string");

        if input_number.len() > 1_000_000 {
            panic!("too many digits");
        }
        
        let palindrome: String = next_palindrome(&input_number);
        
        println!("{}", palindrome);
    }
}

fn read_number() -> u32 {
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Could not parse string");

    let input_number: u32 = match input_number.trim().parse() {
        Ok(number) => number,
        Err(_) => panic!(":-(")
    };

    return input_number;
}

fn next_palindrome(input_number: &String) -> String {
    let mut palindrome: String = input_number.trim().to_string();
    let mut size: usize = palindrome.len();
    let mut all_digits_equal: bool = false;
    let mut checked_until_position: usize = 0;

    for (_idx, c) in palindrome.chars().enumerate() {
        assert!(c.is_digit(10));
    }

    // if the number is already a palindrome, always mutate it
    if check_palindrome(&palindrome) {
        let right_digit_index: usize = size - 1;
        let right_digit: u32 = from_ascii(palindrome.chars().nth(right_digit_index).unwrap());
        increment_digit(&mut palindrome, right_digit, right_digit_index);
        size = palindrome.len(); // might have been changed
    }

    while !all_digits_equal {
        all_digits_equal = true;
        for i in checked_until_position..(size / 2) {
            let right_digit_index: usize = size - 1 - i;
            let left_digit: u32 = from_ascii(palindrome.chars().nth(i).unwrap());
            let right_digit: u32 = from_ascii(palindrome.chars().nth(right_digit_index).unwrap());

            if left_digit == right_digit {
                // println!("fine {} {} {}", palindrome, left_digit, right_digit);
                // nothing to do. Remember that we don't have to check again up to this point
                if checked_until_position == i {
                    checked_until_position += 1;
                }
            } else if left_digit > right_digit {
                // 501 => 505
                // println!("l>r {} {} {}", palindrome, left_digit, right_digit);
                palindrome.replace_range(right_digit_index..right_digit_index + 1, &(left_digit).to_string());
                all_digits_equal = false; // we are not finished yet
            } else {
                // 105 => 109, 199 => 220
                // println!("r>l {} {} {}", palindrome, left_digit, right_digit);
                increment_to(&mut palindrome, left_digit, right_digit_index);
                size = palindrome.len(); // The size might have changed, e.g. with 9 => 10
                all_digits_equal = false; // we are not finished yet
            }
        }
    }
    return palindrome;
}

fn check_palindrome(str_number: &String) -> bool {
    return *str_number == str_number.chars().rev().collect::<String>();
}

// Set the target position to a new digit, increasing the number to left by one (e.g. 14 -> 23, swapping the "4" with a "3")
fn increment_to(palindrome: &mut String, new_digit: u32, position: usize) {
    if position > 0 {
        // 14 => 22
        let neighboring_digit: u32 = from_ascii(palindrome.chars().nth(position - 1).unwrap());
        palindrome.replace_range(position..position + 1, &(new_digit).to_string());

        increment_digit(palindrome, neighboring_digit, position - 1);
    }else{
        // 5 => 14
        let new_string = format!("1{}", new_digit);
        palindrome.replace_range(position..position + 1, &new_string);
    }
}

fn increment_digit(palindrome: &mut String, digit: u32, position: usize) {
    if digit < 9 {
        // 1234 => 1235
        palindrome.replace_range(position..position + 1, &(digit + 1).to_string());
    }else{
        if position > 0 {
            // 1299 => 1300
            let neighboring_digit: u32 = from_ascii(palindrome.chars().nth(position - 1).unwrap());
            palindrome.replace_range(position..position + 1, "0");

            increment_digit(palindrome, neighboring_digit, position - 1);
        }else{
            // 9 => 10
            palindrome.replace_range(position..position + 1, "10");
        }
    }
}

fn from_ascii(ascii_char: char) -> u32 {
    let number: u32 = ascii_char as u32;
    return number - 48; // 49 is 1 in ASCII
}




#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_increment_to() {
        // "Set string "2" at position 1 to 1 (and thus increment its neighbor)"
        let mut x = String::from("123");
        increment_to(&mut x, 1, 1);
        assert_eq!(x, String::from("213"));

        let mut x = String::from("5");
        increment_to(&mut x, 3, 0);
        assert_eq!(x, String::from("13"));

        let mut x = String::from("99");
        increment_to(&mut x, 2, 1);
        assert_eq!(x, String::from("102"));
    }

    #[test]
    fn test_increment_digit() {
        // "Increment string at position 1 (value: 2) by one"
        let mut x = String::from("123");
        increment_digit(&mut x, 2, 1);
        assert_eq!(x, String::from("133"));

        let mut x = String::from("199");
        increment_digit(&mut x, 9, 2);
        assert_eq!(x, String::from("200"));

        let mut x = String::from("9");
        increment_digit(&mut x, 9, 0);
        assert_eq!(x, String::from("10"));

        let mut x = String::from("99");
        increment_digit(&mut x, 9, 1);
        assert_eq!(x, String::from("100"));

        let mut x = String::from("98792798472");
        increment_digit(&mut x, 2, 10);
        assert_eq!(x, String::from("98792798473"));

        let mut x = String::from("999999999999999999999");
        increment_digit(&mut x, 9, 20);
        assert_eq!(x, String::from("1000000000000000000000"));
    }

    #[test]
    fn test_next_palindrome() {
        let mut x = String::from("99");
        assert_eq!(next_palindrome(&mut x), String::from("101"));

        let mut x = String::from("2133");
        assert_eq!(next_palindrome(&mut x), String::from("2222"));

        let mut x = String::from("1287361983619826");
        assert_eq!(next_palindrome(&mut x), String::from("1287361991637821"));

        let mut x = String::from("8080808080");
        assert_eq!(next_palindrome(&mut x), String::from("8080880808"));

        let mut x = String::from("808");
        assert_eq!(next_palindrome(&mut x), String::from("818"));

        let mut x = String::from("999");
        assert_eq!(next_palindrome(&mut x), String::from("1001"));

        let mut x = String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111112");
        assert_eq!(next_palindrome(&mut x), String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111122111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"));
    }
}
