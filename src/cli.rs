use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("sqlant")
        .about(
            "Generate Entity Relationship diagram textual description from SQL connection string",
        )
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("connection_string").required(true))
        .arg(
            Arg::new("inline-puml-lib")
                .long("inline-puml-lib")
                .help("Inline PlantUML lib into diagram code")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("legend")
                .long("legend")
                .help("Add legend to diagram (supported only for PlantUML)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("not_null")
                .short('n')
                .long("nn")
                .help("Add NOT_NULL(NN) marks (for PlantUML always true)")
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
        .arg(
            Arg::new("conceptual")
                .long("conceptual")
                .help("Create conceptual ER diagram")
                .action(ArgAction::SetTrue)
                .default_value("false"),
        )
        .get_matches()
}
