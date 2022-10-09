use sqlant::{get_generator, lookup_parser, GeneratorConfigOptions};
use std::{env, process::exit};

fn print_program_info() {
    println!(
        "Generate PlantUML ER diagram textual description from SQL connection string\n\
        USAGE: sqlant <conn> [OPTIONS]\n\
        \tExample: sqlant postgresql://sqlant_user:sqlant_pswd@localhost/sqlant_db --nn\n\
        OPTIONS:"
    );
    println!("\t-n, --nn {:>42}\n", "Add NOT_NULL(NN) marks");
    println!("\t-e, --en {:>42}\n", "Draw enum types");
}

fn parse_arguments(args: &mut Vec<String>) -> GeneratorConfigOptions {
    let (mut draw_enums, mut not_null) = (false, false);
    while !args.is_empty() {
        let arg = args.pop().unwrap();
        if arg == "-n" || arg == "--nn" {
            not_null = true;
            continue;
        }
        if arg == "-e" || arg == "--en" {
            draw_enums = true;
            continue;
        }
        eprintln!("Unknown argument {:?}", arg);
        exit(2);
    }
    GeneratorConfigOptions {
        not_null,
        draw_enums,
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // Look at clap if you'll need more complicated CLI experience
    // https://docs.rs/clap/latest/clap/
    if args.len() < 2 {
        print_program_info();
        exit(1);
    }

    let mut s = lookup_parser(&args[1]);
    let erd = s.load_erd_data();
    let rndr = get_generator();
    let result = rndr.generate(&erd, &parse_arguments(&mut args.split_off(2)));
    println!("{result}")
}
