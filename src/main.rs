use clap::Parser;
use rand::{self, Rng};

/// Program to show tips about our lord Mogo
#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Print all Mogo tips
    #[clap(short, long)]
    all: bool,
}

const TIPS: &str = include_str!("mogo.txt");

fn main() {
    let args = Args::parse();
    if args.all {
        print!("{}", TIPS);
    } else {
        let lines: Vec<&str> = TIPS.split('\n').collect();
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..lines.len());
        println!("{}", lines[index]);
    }
}
