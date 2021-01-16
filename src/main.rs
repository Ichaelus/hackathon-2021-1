use std::io;
use std::char;
use regex::Regex;

fn main() {
    println!("Booting");
    let rounds: u32 = read_number("rounds");

    println!("Rounds: {}", rounds);
    for _ in 0..rounds {

        let mut input_number = String::new();
        println!("Please insert a number (input string)");
        io::stdin().read_line(&mut input_number).expect("Could not parse string");

        if input_number.len() > 1_000_000 {
            panic!("too many digits");
        }

        let re = Regex::new(r"^\d+$").unwrap();
        assert!(re.is_match(input_number));

        let mut palindrome: String = input_number;

        while !check_palindrome(&palindrome) {
            let digits: Vec<char> = palindrome.chars().collect();

            for i in 0..(digits.len() / 2) {
                let u8 right_digit_index = digits.len() - 1 - i;
                let mut u8 left_digit = match digits[i].parse() {
                    Ok(number) => number,
                    Err(_) => panic!(":-(")
                };
                let mut u8 right_digit = match digits[right_digit_index].parse() {
                    Ok(number) => number,
                    Err(_) => panic!(":-(")
                };

                if left_digit == right_digit {
                    continue; // nothing to do
                }

                if left_digit > right_digit {
                    digits[right_digit_index] = char::from_digit(left_digit); // 21 => 22
                }

                if left_digit < right_digit {
                    increment_palindrome(&digits, i);
                }
            }

            palindrome = digits.to_string();

            // N-th zahl != -N-th zahl
            //   -N-th zahl gleich setzen
            // nächste ziffer betrachten
            //   wenn gleich: nichts zu tun
            //   wenn ungleich:
            //     wenn letzte zahl kleiner als erste: gleich setzen
            //     wenn letzte zahl größer als erste: gleich setzen, und -N-th-1 zahl hochzählen (achtung: rekursiv, sonderfall 9)
            // 1234 => 1235..1239 1240 1241  1251..1291 1301 1331
        }
        
        println!("Found palindrome: {}", palindrome);
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

fn check_palindrome(str_number: String) -> bool {
    let mut is_equal: bool = true;
    let digits: Vec<char> = str_number.chars().collect();

    for i in 0..(digits.len() / 2) {
        is_equal = is_equal && digits[i] == digits[digits.len() - 1 - i];
    }

    return is_equal;
}

fn increment_palindrome(digits: &Vec<char>, position: u32) {
    let u8 right_digit_index = digits.len() - 1 - i;
    let mut u8 neighboring_digit = match digits[right_digit_index - 1].parse() {
        Ok(number) => number,
        Err(_) => panic!(":-(")
    };

    if neighboring_digit < 9 {
        // 19 => 20 ( => 22)
        digits[right_digit_index - 1] = char::from_digit(neighboring_digit + 1);
        digits[right_digit_index] = char::from_digit(0);
    } else {
        // 192 -> 102 -> (recursive) 112  => 202
        digits[right_digit_index - 1] = char::from_digit(0)
        //digits[right_digit_index - 2] = increment_palindrome()
        increment_palindrome(digits, )
    }

    // 109 => 909 statt 111
}




fn check_palindrome_int(number: &u32) -> bool {
    let mut is_equal: bool = true;
    let str_number: String = number.to_string();
    let digits: Vec<char> = str_number.chars().collect();

    for i in 0..(digits.len() / 2) {
        is_equal = is_equal && digits[i] == digits[digits.len() - 1 - i];
    }

    return is_equal;
}
