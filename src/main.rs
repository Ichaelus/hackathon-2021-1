use std::io;

fn main() {
    let mut no_test_cases = String::new();
    io::stdin().read_line(&mut no_test_cases).expect("Failed to get console input");
    let no_test_cases: u32 = no_test_cases.trim().parse().expect("Failed to parse int");

    for _case in 0..no_test_cases {
        let mut no_tuples = String::new();
        io::stdin().read_line(&mut no_tuples).expect("Failed to get console input");

        let mut tuples = String::new();
        io::stdin().read_line(&mut tuples).expect("Failed to get console input");

        let mut password = String::new();
        io::stdin().read_line(&mut password).expect("Failed to get console input");

        let mut empty = String::new();
        io::stdin().read_line(&mut empty).expect("Failed to get console input");
        // for c in empty.chars() {
        //     println!(": {}", c as u8);
        // }
        // assert_eq!("\n", empty);
        // if empty == "\n" {
        //     println!("equal");
        // }

        let tuples = tuples.split_whitespace();

        for tuple in tuples {
            let (a, b) = get_keys(&String::from(tuple));
            print_char_at(&password, a as usize);
            print_char_at(&password, b as usize);
        }
        println!("");
    }
}

fn print_char_at(password: &String, pos: usize) {
    print!("{}", password.chars().nth(pos).unwrap());
}

fn get_keys(key: &String) -> (u32, u32) {
    let mut a: u32 = 0;
    let mut b: u32 = 0;
    
    for (idx, c) in key.chars().enumerate() {
        if c.is_ascii() {
            let c = c as u8;
            if is_bit_set(c, idx as u32) {
                set_bit(&mut a, idx as u32);
            }
            if is_bit_set(c, ((idx as u32)+3)%6) {
                set_bit(&mut b, idx as u32);
            }
        }
    }
    (a,b)
}

fn set_bit(num: &mut u32, pos: u32) {
    let bit = u32::pow(2, pos);
    *num = *num | bit;
}

fn is_bit_set(c: u8, pos: u32) -> bool {
    let val = u8::pow(2, pos);
    let res = c & val;
    if res > 0 {
        true
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_keys() {
        assert_eq!(get_keys(&String::from("qwe345")), (55, 46));
    }
}