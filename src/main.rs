mod plantuml_renderer;
mod psql_er_parser;
mod sql_entities;

use plantuml_renderer::PlantUmlDefaultRenderer;
use postgres::{Client, NoTls};
use psql_er_parser::PostgreSqlERParser;
use sql_entities::{PlantUmlRenderer, SqlERDataLoader, Table};
use std::env;
use std::result::Result;

fn lookup_parser(connection_string: &str) -> Box<dyn SqlERDataLoader> {
    if connection_string.starts_with("postgresql") {
        return Box::new(PostgreSqlERParser::new(connection_string));
    }
    panic!("Appropriate parser is not found :(");
}

fn get_renderer() -> Box<dyn PlantUmlRenderer> {
    Box::new(PlantUmlDefaultRenderer::new())
}

fn main() {
    // Two types of modules loaders and renderers

    let args: Vec<String> = env::args().collect();
    // TODO make better error printing. Maybe use clap?
    // https://docs.rs/clap/latest/clap/
    assert!(args.len() == 2, "Usage: sqlant <conn>");
    let connection_string = &args[1];

    // parseArgs
    // getTablesFromDB
    let mut s = lookup_parser(connection_string);
    let mut erd = s.load_erd_data();

    let rndr = get_renderer();
    let result = rndr.render(&erd);
    println!("{result}")
    // rndr.render();
    // <rndr as PlantUmlRenderer>::render(&erd);

    // filter tables
}
