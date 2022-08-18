use postgres::types::FromSql;
use postgres::{Client, NoTls};
use std::result::Result;

mod psql_er_parser;
mod sql_entities;
use psql_er_parser::PostgreSqlERParser;
use sql_entities::{SqlERDataLoader, Table};

fn lookup_parser(connection_string: &str) -> Box<dyn SqlERDataLoader> {
    if connection_string.starts_with("postgresql") {
        return Box::new(PostgreSqlERParser::new(connection_string));
    }
    panic!("Appropriate parser is not found :(");
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO make better error printing. Maybe use clap?
    // https://docs.rs/clap/latest/clap/
    assert!(args.len() == 2, "Usage: sqlant <conn>");
    let connection_string = &args[1];

    // parseArgs
    // getTablesFromDB
    let mut s = lookup_parser(connection_string);
    let mut erd = s.load_erd_data();

    // filter tables
}
