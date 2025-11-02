use std::sync::Arc;

use super::sql_entities::{SqlERData, Table, TableColumn};
use crate::{Direction, GeneratorConfigOptions, ViewGenerator};
use serde::Serialize;
use tinytemplate::{TinyTemplate, format_unescaped};

static MERMAID_TEMPLATE: &str = r#"erDiagram
{{ if direction }}direction {direction}{{ endif }}
{{ for ent in entities}}{ent}{{ endfor }}
{{ for en in enums}}{en}{{ endfor }}
{{ for fk in foreign_keys}}{fk}{{ endfor }}
"#;

static ENTITY_TEMPLATE: &str = "{name} \\{\n{pks}{fks}{others}}\n";

static COLUMN_TEMPLATE: &str = "    {col.datatype} {col.name}{{ if is_pk_or_fk }} {{ endif }}{{ if is_pk }}PK,{{ endif }}{{ if is_fk }}FK{{ endif }}";

static REL_TEMPLATE: &str = "{source_table_name} {{ if is_zero_one_to_one }}|o--||{{else}}}o--||{{ endif }} {target_table_name}: \"\"\n";

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
enum SDirection {
    TB,
    BT,
    LR,
    RL,
}

impl From<&Direction> for SDirection {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::TB => Self::TB,
            Direction::BT => Self::BT,
            Direction::LR => Self::LR,
            Direction::RL => Self::RL,
        }
    }
}

#[derive(Serialize)]
struct SMermaid {
    direction: Option<SDirection>,
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
    pub fn new() -> Result<MermaidGenerator<'a>, crate::SqlantError> {
        let mut str_templates = TinyTemplate::new();
        str_templates.add_template("mermaid", MERMAID_TEMPLATE)?;
        str_templates.add_template("column", COLUMN_TEMPLATE)?;
        str_templates.add_template("ent", ENTITY_TEMPLATE)?;
        str_templates.add_template("rel", REL_TEMPLATE)?;
        str_templates.add_template("enum", ENUM_TEMPLATE)?;
        str_templates.set_default_formatter(&format_unescaped);
        Ok(MermaidGenerator { str_templates })
    }

    fn entity_render(
        &self,
        tbl: &Table,
        opts: &GeneratorConfigOptions,
    ) -> Result<String, crate::SqlantError> {
        enum RenderType {
            PK,     // only pk columns
            FK,     // only pure FK columns (Non PK)
            Others, // non pk and non fk
        }
        let columns_render = |rt: RenderType| {
            tbl.columns
                .iter()
                .filter(|col| match rt {
                    RenderType::PK => col.is_pk(),
                    RenderType::FK => !col.is_pk() && col.is_fk(),
                    RenderType::Others => !col.is_pk() && !col.is_fk(),
                })
                .try_fold(String::new(), |acc, col| {
                    let column = &SColumn {
                        col: col.as_ref(),
                        is_fk: col.is_fk(),
                        is_pk: col.is_pk(),
                        is_pk_or_fk: col.is_pk() || col.is_fk(),
                        is_nn: opts.not_null && col.is_nn(),
                    };
                    let mut res: String = self
                        .str_templates
                        .render("column", &column)?
                        .trim_end_matches([','])
                        .into();
                    if column.is_nn {
                        res += " \"NN\"";
                    }

                    Ok::<std::string::String, crate::SqlantError>(acc + &res + "\n")
                })
        };
        Ok(self.str_templates.render(
            "ent",
            &SEntity {
                pks: columns_render(RenderType::PK)?,
                fks: columns_render(RenderType::FK)?,
                others: columns_render(RenderType::Others)?,
                name: tbl.name.clone(),
            },
        )?)
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

impl ViewGenerator for MermaidGenerator<'_> {
    fn generate(
        &self,
        mut sql_erd: SqlERData,
        opts: &GeneratorConfigOptions,
    ) -> Result<String, crate::SqlantError> {
        Self::preprocess(&mut sql_erd);
        let entities: Vec<String> = sql_erd
            .tables
            .iter()
            .map(|tbl| self.entity_render(tbl, opts))
            .collect::<Result<Vec<String>, crate::SqlantError>>()?;
        let foreign_keys: Vec<String> = sql_erd
            .foreign_keys
            .iter()
            .map(|fk| {
                self.str_templates.render(
                    "rel",
                    &SForeignKey {
                        source_table_name: fk.source_table.name.clone(),
                        target_table_name: fk.target_table.name.clone(),
                        is_zero_one_to_one: fk.is_zero_one_to_one,
                    },
                )
            })
            .collect::<Result<Vec<String>, _>>()?;

        let enums: Vec<String> = if opts.draw_enums {
            sql_erd
                .enums
                .iter()
                .map(|(name, values)| {
                    self.str_templates.render(
                        "enum",
                        &SEnum {
                            name: name.to_string(),
                            values: values.to_vec(),
                        },
                    )
                })
                .collect::<Result<Vec<String>, _>>()?
        } else {
            vec![]
        };

        Ok(self.str_templates.render(
            "mermaid",
            &SMermaid {
                direction: opts.direction.as_ref().map(Into::into),
                entities,
                enums,
                foreign_keys,
            },
        )?)
    }
}
