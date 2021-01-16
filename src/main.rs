fn main() {
    println!("Hello, world!");
}

fn next_palin(x: &mut String) {
    if x.len()%2 == 0 {
        let a = x[..(x.len()/2)];
        let b = x[(x.len()/2)..];

        if is_less_or_equal_than(a, b) {
            increment(a);
        }
        x[(x.len()/2)..] = reverse(a);
    } else {

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

fn is_less_than(a: String, b: String) -> bool {
    // a: 123122031
    // b: 123123123
    assert_eq!(a.len(), b.len());

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
    fn test_is_less_than() {
        assert!(is_less_than(String::from("1"), String::from("2")));

        assert!(is_less_than(String::from("0001"), String::from("1000")));

        assert!(is_less_than(String::from("09998"), String::from("10000")));

        assert!(!is_less_than(String::from("09998"), String::from("09997")));
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
}