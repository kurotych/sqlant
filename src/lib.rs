pub mod plantuml_renderer;
pub mod psql_erd_loader;
pub mod sql_entities;

use plantuml_renderer::PlantUmlDefaultRenderer;
use psql_erd_loader::PostgreSqlERDLoader;
use sql_entities::{PlantUmlRenderer, SqlERDataLoader};

pub fn lookup_parser(connection_string: &str) -> Box<dyn SqlERDataLoader> {
    if connection_string.starts_with("postgresql") {
        return Box::new(PostgreSqlERDLoader::new(connection_string));
    }
    panic!("Appropriate parser is not found :(");
}

// If you want to add renderer, you need to add input parameter
// for this function.
// To distinguish what exactly renderer you need.
pub fn get_renderer() -> Box<dyn PlantUmlRenderer> {
    Box::new(PlantUmlDefaultRenderer::new())
}
