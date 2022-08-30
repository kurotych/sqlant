pub mod plantuml_generator;
pub mod psql_erd_loader;
pub mod sql_entities;

use plantuml_generator::PlantUmlDefaultGenerator;
use psql_erd_loader::PostgreSqlERDLoader;
use sql_entities::{PlantUmlGenerator, SqlERDataLoader};

pub fn lookup_parser(connection_string: &str) -> Box<dyn SqlERDataLoader> {
    if connection_string.starts_with("postgresql") {
        return Box::new(PostgreSqlERDLoader::new(connection_string));
    }
    panic!("Appropriate parser is not found :(");
}

// If you want to add generator, you need to add input parameter
// for this function.
// To distinguish what exactly generator you need.
pub fn get_generator() -> Box<dyn PlantUmlGenerator> {
    Box::new(PlantUmlDefaultGenerator::new())
}
