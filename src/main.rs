use sqlant::{get_generator, lookup_parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Look at clap if you'll need more complicated CLI experience
    // https://docs.rs/clap/latest/clap/
    assert!(args.len() == 2, "Usage: sqlant <conn>");
    let mut s = lookup_parser(&args[1]);
    let erd = s.load_erd_data();
    let rndr = get_generator();
    let result = rndr.generate(&erd);
    println!("{result}")
}
