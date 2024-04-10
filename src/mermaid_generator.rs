use std::sync::Arc;

use super::sql_entities::{SqlERData, Table, TableColumn};
use crate::{GeneratorConfigOptions, ViewGenerator};
use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

static MERMAID_TEMPLATE: &str = r#"erDiagram
{{ for ent in entities}}{ent}{{ endfor }}
{{ for en in enums}}{en}{{ endfor }}
{{ for fk in foreign_keys}}{fk}{{ endfor }}
"#;

static ENTITY_TEMPLATE: &str = "{name} \\{\n{pks}{fks}{others}}\n";

static COLUMN_TEMPLATE: &str =
    "    {col.datatype} {col.name}{{ if is_pk_or_fk }} {{ endif }}{{ if is_pk }}PK,{{ endif }}{{ if is_fk }}FK{{ endif }}";

static REL_TEMPLATE: &str =
    "{source_table_name} {{ if is_zero_one_to_one }}|o--||{{else}}}o--||{{ endif }} {target_table_name}: \"\"\n";

const ENUM_TEMPLATE: &str = "\"{name} (ENUM)\" \\{\n{{ for v in values}}    {v} _\n{{ endfor }}}";

#[derive(Serialize)]
struct SEntity {
    name: String,
    pks: String,    // Columns that contain PK
    fks: String,    // Columns that contain FK and don't contain PK
    others: String, // Columns that don't contain both PK and FK
}

#[derive(Serialize)]
struct SColumn<'a> {
    col: &'a TableColumn,
    is_fk: bool,
    is_pk: bool,
    is_pk_or_fk: bool,
    is_nn: bool,
}

#[derive(Serialize)]
struct SMermaid {
    entities: Vec<String>,
    enums: Vec<String>,
    foreign_keys: Vec<String>,
}

#[derive(Serialize)]
struct SForeignKey {
    source_table_name: String,
    target_table_name: String,
    is_zero_one_to_one: bool,
}

#[derive(Serialize)]
struct SEnum {
    name: String,
    values: Vec<String>,
}

pub struct MermaidGenerator<'a> {
    str_templates: TinyTemplate<'a>,
}

impl<'a> MermaidGenerator<'a> {
    pub fn new() -> MermaidGenerator<'a> {
        let mut str_templates = TinyTemplate::new();
        str_templates
            .add_template("mermaid", MERMAID_TEMPLATE)
            .unwrap();
        str_templates
            .add_template("column", COLUMN_TEMPLATE)
            .unwrap();
        str_templates.add_template("ent", ENTITY_TEMPLATE).unwrap();
        str_templates.add_template("rel", REL_TEMPLATE).unwrap();
        str_templates.add_template("enum", ENUM_TEMPLATE).unwrap();
        str_templates.set_default_formatter(&format_unescaped);
        MermaidGenerator { str_templates }
    }

    fn entity_render(&self, tbl: &Table, opts: &GeneratorConfigOptions) -> String {
        // if pk_render - render only pk columns
        // if fk_render - render only pure FK columns (Non PK)
        // if both are false return non pk and non fk
        let columns_render = |pk_render: bool, fk_render: bool| {
            tbl.columns
                .iter()
                .filter(|col| {
                    if pk_render {
                        return col.is_pk();
                    } // otherwise render non pk columns
                    if fk_render {
                        return !col.is_pk() && col.is_fk();
                    }
                    if !pk_render && !fk_render {
                        return !col.is_pk() && !col.is_fk();
                    }
                    panic!("Aaa! Something went wrong!");
                })
                .fold(String::new(), |acc, col| {
                    let column = &SColumn {
                        col: col.as_ref(),
                        is_fk: col.is_fk(),
                        is_pk: col.is_pk(),
                        is_pk_or_fk: col.is_pk() || col.is_fk(),
                        is_nn: opts.not_null && col.is_nn(),
                    };
                    let mut res: String = self
                        .str_templates
                        .render("column", &column)
                        .unwrap()
                        .trim_end_matches(|c| c == ',')
                        .into();
                    if column.is_nn {
                        res += " \"NN\"";
                    }

                    acc + &res + "\n"
                })
        };
        self.str_templates
            .render(
                "ent",
                &SEntity {
                    pks: columns_render(true, false),
                    fks: columns_render(false, true),
                    others: columns_render(false, false),
                    name: tbl.name.clone(),
                },
            )
            .unwrap()
    }

    // Preprocess sql_erd data to make it compatible with mermaid ERD
    fn preprocess(sql_erd: &mut SqlERData) {
        for table in sql_erd.tables.iter_mut() {
            let tbl = Arc::make_mut(table);
            for c in &mut tbl.columns {
                let c = Arc::make_mut(c);
                let replaced_string = c.datatype.replace(' ', "_");
                c.datatype = replaced_string;
            }
        }
    }
}

impl<'a> ViewGenerator for MermaidGenerator<'a> {
    fn generate(&self, mut sql_erd: SqlERData, opts: &GeneratorConfigOptions) -> String {
        Self::preprocess(&mut sql_erd);
        let entities: Vec<String> = sql_erd
            .tables
            .iter()
            .map(|tbl| self.entity_render(tbl, opts))
            .collect();
        let foreign_keys: Vec<String> = sql_erd
            .foreign_keys
            .iter()
            .map(|fk| {
                self.str_templates
                    .render(
                        "rel",
                        &SForeignKey {
                            source_table_name: fk.source_table.name.clone(),
                            target_table_name: fk.target_table.name.clone(),
                            is_zero_one_to_one: fk.is_zero_one_to_one,
                        },
                    )
                    .unwrap()
            })
            .collect();

        let enums: Vec<String> = if opts.draw_enums {
            sql_erd
                .enums
                .iter()
                .map(|(name, values)| {
                    self.str_templates
                        .render(
                            "enum",
                            &SEnum {
                                name: name.to_string(),
                                values: values.to_vec(),
                            },
                        )
                        .unwrap()
                })
                .collect()
        } else {
            vec![]
        };

        self.str_templates
            .render(
                "mermaid",
                &SMermaid {
                    entities,
                    enums,
                    foreign_keys,
                },
            )
            .unwrap()
    }
}
