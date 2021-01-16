use std::io;

fn main() {
    let mut no_test_cases = String::new();
    io::stdin().read_line(&mut no_test_cases).expect("Failed to get console input");
    let no_test_cases: u32 = no_test_cases.trim().parse().expect("Failed to parse int");

    for _case in 0..no_test_cases {
        let mut test_case = String::new();
        io::stdin().read_line(&mut test_case).expect("Failed to get console input");
        test_case = test_case.trim().to_string();
        let palin = next_palin(&mut test_case);
        println!("{}", palin);
    }
}

fn next_palin(x: &mut String) -> String {
    if x.len()%2 == 0 {
        let (a, b) = x.split_at(x.len()/2);
        let mut a = String::from(a);
        let b = String::from(b);

        let mut carry = false;
        if need_increment(&a, &reverse(&b)) {
           carry = increment(&mut a);
        }
        a.push_str( &reverse(&a));

        if carry {
            a.remove(1);
        }

        return a;
    } else {
        let (a, b) = x.split_at(x.len()/2);
        let (m, b) = b.split_at(1);
        let mut a = String::from(a);
        let m = String::from(m);
        let b = String::from(b);

        let mut carry = false;
        let mut m: u32 = m.parse().expect("no digit");
        if need_increment(&a, &reverse(&b)) {
            m = (m +1)%10;

            if m == 0 {
                carry = increment(&mut a);
            }
        }

        let mut m = m.to_string();
        m.push_str(&reverse(&a));
        a.push_str(&m.to_string());
        
        if carry {
            a.remove(1);
        }
        return a;
    }
}


fn reverse(x: &String) -> String {
    x.chars().rev().collect::<String>()
}


fn increment(x: &mut String) -> bool {
    let mut carry = false;
    for pos in 0..x.len() {
        let pos = x.len() - (pos+1);

        let c = x.chars().nth(pos).unwrap();
        assert!(c.is_digit(10));
        let mut c = c.to_digit(10).unwrap();

        if c < 9 {
            c = c + 1;
            carry = false;
        } else {
            c = 0;
            carry = true;
        }
        let c = c.to_string();
        x.replace_range(pos..pos+1, &c);
        if !carry {
            break;
        }
    }
    if carry {
        x.insert_str(0, "1");
    }
    return carry;
}

fn need_increment(a: &String, b: &String) -> bool {
    assert_eq!(a.len(), b.len());
    
    let a_rev = reverse(a);
    if &a_rev == b { return true; }

    for pos in 0..a.len() {
        let a_digit = a_rev.chars().nth(pos).unwrap();
        let b_digit = b.chars().nth(pos).unwrap();

        let a_digit = a_digit.to_digit(10).unwrap();
        let b_digit = b_digit.to_digit(10).unwrap();

        if a_digit < b_digit {
            return true;
        } else if a_digit > b_digit {
            return false;
        }
    }

    return true;
}

fn is_less_or_equal_than(a: &String, b: &String) -> bool {
    // a: 123122031
    // b: 123123123
    assert_eq!(a.len(), b.len());

    if a == b { return true; }

    for pos in 0..a.len() {
        let a = a.chars().nth(pos).unwrap();
        let b = b.chars().nth(pos).unwrap();
        assert!(a.is_digit(10));
        assert!(b.is_digit(10));

        let a = a.to_digit(10).unwrap();
        let b = b.to_digit(10).unwrap();

        if a < b {
            return true;
        }
    }
    return false;

}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_less_or_equal_than() {
        assert!(is_less_or_equal_than(&String::from("1"), &String::from("2")));

        assert!(is_less_or_equal_than(&String::from("0001"), &String::from("1000")));

        assert!(is_less_or_equal_than(&String::from("09998"), &String::from("10000")));

        assert!(!is_less_or_equal_than(&String::from("9998"), &String::from("9997")));

        assert!(is_less_or_equal_than(&String::from("9998"), &String::from("9998")));
    }

    #[test]
    fn test_increment() {
        let mut x = String::from("1");
        increment(&mut x);
        assert_eq!(x, String::from("2"));

        let mut x = String::from("9");
        increment(&mut x);
        assert_eq!(x, String::from("10"));

        let mut x = String::from("98792798472");
        increment(&mut x);
        assert_eq!(x, String::from("98792798473"));

        let mut x = String::from("999999999999999999999");
        increment(&mut x);
        assert_eq!(x, String::from("1000000000000000000000"));
    }

    #[test]
    fn test_reverse() {
        let x = String::from("abcde");
        assert_eq!(reverse(&x), String::from("edcba"));
    }

    #[test]
    fn test_next_palin() {
        let mut x = String::from("99");
        assert_eq!(next_palin(&mut x), String::from("101"));

        let mut x = String::from("2133");
        assert_eq!(next_palin(&mut x), String::from("2222"));

        let mut x = String::from("1287361983619826");
        assert_eq!(next_palin(&mut x), String::from("1287361991637821"));

        let mut x = String::from("8080808080");
        assert_eq!(next_palin(&mut x), String::from("8080880808"));

        let mut x = String::from("808");
        assert_eq!(next_palin(&mut x), String::from("818"));

        let mut x = String::from("999");
        assert_eq!(next_palin(&mut x), String::from("1001"));

        let mut x = String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111112");
        assert_eq!(next_palin(&mut x), String::from("11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111122111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111"));

    }
}