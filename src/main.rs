use rand::{self, Rng};

const TIPS: &str = include_str!("mogo.txt");

fn main() {
    let lines: Vec<&str> = TIPS.split('\n').collect();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..lines.len());
    println!("{}", lines[index]);
}
