use std::str::FromStr;

use clap::ArgMatches;
use sqlant::{get_generator, lookup_parser, GeneratorConfigOptions, GeneratorType};

fn get_arg(args: &ArgMatches, arg_name: &str) -> String {
    args.get_one::<String>(arg_name).unwrap().to_string()
}

#[tokio::main]
async fn main() {
    let args = sqlant::cli::parse();

    let mut s = lookup_parser(
        &get_arg(&args, "connection_string"),
        get_arg(&args, "schema"),
    )
    .await
    .unwrap();
    let erd = s.load_erd_data().await.unwrap();
    let output_arg = get_arg(&args, "output");
    let generator_type =
        GeneratorType::from_str(&output_arg).expect("Generator type {output_arg} isn't supported");
    let rndr = get_generator(generator_type).unwrap();
    let result = rndr
        .generate(
            erd,
            &GeneratorConfigOptions {
                not_null: args.get_flag("not_null"),
                draw_enums: args.get_flag("enums"),
                draw_legend: args.get_flag("legend"),
                inline_puml_lib: args.get_flag("inline-puml-lib"),
                conceptual_diagram: args.get_flag("conceptual"),
            },
        )
        .unwrap();
    println!("{result}")
}
