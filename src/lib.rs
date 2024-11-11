use strum_macros::{Display, EnumString};

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
    pub draw_legend: bool,
    pub inline_puml_lib: bool,
}

pub trait ViewGenerator {
    fn generate(
        &self,
        sql_erd: SqlERData,
        opts: &GeneratorConfigOptions,
    ) -> Result<String, SqlantError>;
}

pub async fn lookup_parser(
    connection_string: &str,
    schema_name: String,
) -> Result<Box<dyn SqlERDataLoader>, SqlantError> {
    Ok(Box::new(
        PostgreSqlERDLoader::new(connection_string, schema_name).await?,
    ))
}

#[derive(Clone, Debug, Display, EnumString, Eq, PartialEq, PartialOrd, Ord)]
#[strum(serialize_all = "lowercase")]
pub enum GeneratorType {
    PlantUML,
    Mermaid,
}

// If you want to add generator, you need to add input parameter
// for this function.
// To distinguish what exactly generator you need.
pub fn get_generator(generator_type: GeneratorType) -> Result<Box<dyn ViewGenerator>, SqlantError> {
    match generator_type {
        GeneratorType::PlantUML => Ok(Box::new(PlantUmlDefaultGenerator::new()?)),
        GeneratorType::Mermaid => Ok(Box::new(MermaidGenerator::new()?)),
    }
}
