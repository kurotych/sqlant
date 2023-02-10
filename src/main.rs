use clap::{Arg, ArgAction, Command};
use sqlant::{get_generator, lookup_parser, GeneratorConfigOptions};

fn main() {
    let args = Command::new("sqlant")
        .about("Generate PlantUML ER diagram textual description from SQL connection string")
        .arg(Arg::new("connection_string").required(true))
        .arg(
            Arg::new("not_null")
                .short('n')
                .long("nn")
                .help("Add NOT_NULL(NN) marks")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("enums")
                .short('e')
                .long("en")
                .help("Draw enum types")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let mut s = lookup_parser(args.get_one::<String>("connection_string").unwrap());
    let erd = s.load_erd_data();
    let rndr = get_generator();
    let result = rndr.generate(
        &erd,
        &GeneratorConfigOptions {
            not_null: args.get_flag("not_null"),
            draw_enums: args.get_flag("enums"),
        },
    );
    println!("{result}")
}
