use std::io;

fn main() {
    let mut sum = 0;
    let mut round = 2;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not parse");

        let input: u32 = match input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("no thanks");
                return;
            }
        };

        if input > 200 {
           println!("Invalid number, too big!");
           continue;
        }

        sum += input;
        round -= 1;

        if round == 0 {
            break;
        }
    }
    println!("{}", sum);
}
