use sqlant::{get_renderer, lookup_parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Look at clap if you'll need more complicated CLI experience
    // https://docs.rs/clap/latest/clap/
    assert!(args.len() == 2, "Usage: sqlant <conn>");
    let mut s = lookup_parser(&args[1]);
    let erd = s.load_erd_data();
    let rndr = get_renderer();
    let result = rndr.render(&erd);
    println!("{result}")
}
