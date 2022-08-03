use postgres::types::FromSql;
use postgres::{Client, NoTls};
use std::result::Result;

mod psql_er_parser;
mod sql_entities;
use psql_er_parser::PostgreSqlERParser;
use sql_entities::{SqlERDataLoader, Table};

// get list of tables and its oid
// SELECT table_name, table_name::regclass::oid as table_oid FROM information_schema.tables where
// table_schema = 'public';

// Obtain foreign keys
/*
SELECT conrelid::regclass as "source_table_name",
       confrelid::regclass as "target_table_name",
       conname AS foreign_key_name,
       oid as "foreign_key_oid",
       conkey as "source_column_nums",
       confkey as "target_columns_nums",
       conrelid, -- "source_table_oid(conrelid)",
       confrelid as "target_table_oid(confrelid)",
       pg_get_constraintdef(oid) -- just for debugging purposes
INTO TEMP TABLE temp_table
FROM   pg_constraint
WHERE  contype = 'f'
AND    connamespace = 'public'::regnamespace
ORDER  BY source_table_name;
*/

// To retrieve column names:
// select attname from pg_attribute where attrelid = 16418 and attnum = 2;

//SELECT conrelid::regclass AS table_name,
//       conname AS foreign_key,
//       oid, conrelid,
//              pg_get_constraintdef(oid)
//              FROM   pg_constraint
//              WHERE  contype = 'f'
//              AND    connamespace = 'public'::regnamespace
//              ORDER  BY conrelid::regclass::text, contype DESC

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
