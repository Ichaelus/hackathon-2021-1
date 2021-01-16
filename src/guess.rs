use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut rng = thread_rng();
    let mut y = ["Thomas", "Julian", "Andreas", "Michael", "Emanuel", "Stefan"];
    println!("Unshuffled: {:?}", y);
    y.shuffle(&mut rng);
    println!("Shuffled:   {:?}", y);

    for pos in 0..y.len()/2 {
        println!("{}, {}", y[pos*2], y[pos*2+1]);
    }
}