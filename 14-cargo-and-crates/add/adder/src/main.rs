use add_one::add_one;
use rand::Rng;
fn main() {
    let num = 10;
    
    let rand = rand::thread_rng().gen_range(1..100);

    println!(
        "Thee number {} plus one is {}",
        num, add_one(num)
    );

    println!("Random number: {}", rand);
}
