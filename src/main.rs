use clap::{Arg, ArgAction, ArgMatches, Command};
use sqlant::{get_generator, lookup_parser, GeneratorConfigOptions};

fn get_arg(args: &ArgMatches, arg_name: &str) -> String {
    args.get_one::<String>(arg_name).unwrap().to_string()
}

fn main() {
    let args = Command::new("sqlant")
        .about(
            "Generate Entity Relationship diagram textual description from SQL connection string",
        )
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
        .arg(
            Arg::new("schema")
                .short('s')
                .long("schema")
                .help("Schema name")
                .action(ArgAction::Set)
                .default_value("public"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .value_parser(["plantuml", "mermaid"])
                .long("output")
                .help("Generate output in mermaid format")
                .action(ArgAction::Set)
                .default_value("plantuml"),
        )
        .get_matches();

    let mut s = lookup_parser(
        &get_arg(&args, "connection_string"),
        get_arg(&args, "schema"),
    )
    .unwrap();
    let erd = s.load_erd_data();
    let rndr = get_generator(&get_arg(&args, "output"));
    let result = rndr.generate(
        erd,
        &GeneratorConfigOptions {
            not_null: args.get_flag("not_null"),
            draw_enums: args.get_flag("enums"),
        },
    );
    println!("{result}")
}
