pub mod error;
pub mod mermaid_generator;
pub mod plantuml_generator;
pub mod psql_erd_loader;
pub mod sql_entities;

pub use error::SqlantError;
use mermaid_generator::MermaidGenerator;
use plantuml_generator::PlantUmlDefaultGenerator;
use psql_erd_loader::PostgreSqlERDLoader;
use sql_entities::{SqlERData, SqlERDataLoader};

pub struct GeneratorConfigOptions {
    pub not_null: bool,
    pub draw_enums: bool,
}

pub trait ViewGenerator {
    fn generate(&self, sql_erd: SqlERData, opts: &GeneratorConfigOptions) -> String;
}

pub fn lookup_parser(
    connection_string: &str,
    schema_name: String,
) -> Result<Box<dyn SqlERDataLoader>, SqlantError> {
    Ok(Box::new(PostgreSqlERDLoader::new(
        connection_string,
        schema_name,
    )?))
}

// If you want to add generator, you need to add input parameter
// for this function.
// To distinguish what exactly generator you need.
pub fn get_generator(generator_type: &str) -> Box<dyn ViewGenerator> {
    match generator_type {
        "plantuml" => Box::new(PlantUmlDefaultGenerator::new()),
        "mermaid" => Box::new(MermaidGenerator::new()),
        _ => panic!("Generator type {generator_type} isn't supported"),
    }
}
