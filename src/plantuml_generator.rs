use super::sql_entities::{SqlERData, Table, TableColumn};
use crate::{GeneratorConfigOptions, ViewGenerator};
use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

pub struct PlantUmlDefaultGenerator<'a> {
    str_templates: TinyTemplate<'a>,
}

static PUML_TEMPLATE: &str = "@startuml \n\n\
    hide circle\n\
    skinparam linetype ortho\n\n\
    {{ for ent in entities}}{ent}\n{{ endfor }}\n\
    {{ for fk in foreign_keys}}{fk}\n{{ endfor }}\n\
    {{ for e in enums}}{e}\n{{ endfor }}@enduml\n";

static ENTITY_TEMPLATE: &str = "\"**{name}**\" \\{\n{pks}---\n{fks}{others}}\n";

static COLUMN_TEMPLATE: &str =
    "{{ if is_pk }}#{{else}}*{{ endif }} <b>\"\"{col.name}\"\"</b>: //\"\"{col.datatype}\"\" \
        {{ if is_pk }}<b><color:goldenrod>(PK) </color></b>{{ endif }}{{ if is_fk }}<b><color:701fc6>(FK) </color></b>{{ endif }}\
        {{ if is_nn }}<b><color:DarkRed>(NN) </color></b>{{ endif }} //\n";

static REL_TEMPLATE: &str =
    "\"**{source_table_name}**\" {{ if is_zero_one_to_one }}|o--||{{else}}}o--||{{ endif }} \"**{target_table_name}**\"\n";

static ENUM_TEMPLATE: &str =
    "object \"<color:BlueViolet>**{name}**</color> (enum)\" as {name} \\{\n{{ for v in values}} {v}\n{{ endfor }}}\n";

#[derive(Serialize)]
struct SSqlEnum {
    name: String,
    values: Vec<String>,
}

#[derive(Serialize)]
struct Entities {
    entities: Vec<String>,
}

#[derive(Serialize)]
struct SColumn<'a> {
    col: &'a TableColumn,
    is_fk: bool,
    is_pk: bool,
    is_nn: bool,
}

#[derive(Serialize)]
struct SEntity {
    name: String,
    pks: String,    // Columns that contain PK
    fks: String,    // Columns that contain FK and don't containt PK
    others: String, // Columns that don't contain both PK and FK
}

#[derive(Serialize)]
struct SPuml {
    entities: Vec<String>,
    foreign_keys: Vec<String>,
    enums: Vec<String>,
}

#[derive(Serialize)]
struct SForeignKey {
    source_table_name: String,
    target_table_name: String,
    is_zero_one_to_one: bool,
}

impl<'a> PlantUmlDefaultGenerator<'a> {
    pub fn new() -> PlantUmlDefaultGenerator<'a> {
        let mut str_templates = TinyTemplate::new();
        str_templates.add_template("puml", PUML_TEMPLATE).unwrap();
        str_templates.add_template("pk", COLUMN_TEMPLATE).unwrap();
        str_templates.add_template("ent", ENTITY_TEMPLATE).unwrap();
        str_templates.add_template("rel", REL_TEMPLATE).unwrap();
        str_templates.add_template("enum", ENUM_TEMPLATE).unwrap();
        str_templates.set_default_formatter(&format_unescaped);
        PlantUmlDefaultGenerator { str_templates }
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
                    acc + &self
                        .str_templates
                        .render(
                            "pk",
                            &SColumn {
                                col: col.as_ref(),
                                is_fk: col.is_fk(),
                                is_pk: col.is_pk(),
                                is_nn: opts.not_null && col.is_nn(),
                            },
                        )
                        .unwrap()
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
}

impl<'a> ViewGenerator for PlantUmlDefaultGenerator<'a> {
    fn generate(&self, sql_erd: SqlERData, opts: &GeneratorConfigOptions) -> String {
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
                            &SSqlEnum {
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
                "puml",
                &SPuml {
                    entities,
                    foreign_keys,
                    enums,
                },
            )
            .unwrap()
    }
}
