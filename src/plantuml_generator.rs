use super::sql_entities::{SqlERData, Table, TableColumn};
use crate::{GeneratorConfigOptions, ViewGenerator};
use serde::Serialize;
use tinytemplate::{format_unescaped, TinyTemplate};

pub struct PlantUmlDefaultGenerator<'a> {
    str_templates: TinyTemplate<'a>,
}

static PUML_TEMPLATE: &str = "@startuml\n\n\
    hide circle\n\
    skinparam linetype ortho\n\n\
    {{ for ent in entities}}{ent}\n{{ endfor }}\n\
    {{ for fk in foreign_keys}}{fk}\n{{ endfor }}\n\
    {{ for e in enums}}{e}\n{{ endfor }}{legend}@enduml\n";

static ENTITY_TEMPLATE: &str = "entity \"**{name}**\" \\{\n{pks}---\n{fks}{others}}\n";

static COLUMN_TEMPLATE: &str =
    "{{ if is_nn_and_not_pk }}*{{ endif }}{{ if is_pk }}<b><color:#d99d1c><&key></color>{{else}}{{ endif }}{{ if is_fk }}<color:#aaaaaa><&key></color>{{ endif }}<b>\"\"{col.name}\"\"</b>: //\"\"{col.datatype}\"\" //\n";

static REL_TEMPLATE: &str =
    "\"**{source_table_name}**\" {{ if is_zero_one_to_one }}|o--||{{else}}}o--||{{ endif }} \"**{target_table_name}**\"\n";

static ENUM_TEMPLATE: &str =
    "object \"<color:BlueViolet>**{name}**</color> (enum)\" as {name} \\{\n{{ for v in values}} {v}\n{{ endfor }}}\n";

static PUML_LEGEND: &str = r#"
legend right
 <#GhostWhite,#GhostWhite>|   |= __Legend__ |
 |<b><color:#b8861b><&key></color></b>| Primary Key |
 |<color:#aaaaaa><&key></color>| Foreign Key |
 | &#8226; | Mandatory field (Not Null) |
endlegend
"#;

#[derive(Serialize)]
struct SSqlEnum {
    name: String,
    values: Vec<String>,
}

#[derive(Serialize)]
struct SColumn<'a> {
    col: &'a TableColumn,
    is_fk: bool,
    is_pk: bool,
    is_nn: bool,
    is_nn_and_not_pk: bool,
}

#[derive(Serialize)]
struct SEntity {
    name: String,
    pks: String,    // Columns that contain PK
    fks: String,    // Columns that contain FK and don't contain PK
    others: String, // Columns that don't contain both PK and FK
}

#[derive(Serialize)]
struct SLegend(String);

#[derive(Serialize)]
struct SPuml {
    entities: Vec<String>,
    foreign_keys: Vec<String>,
    enums: Vec<String>,
    legend: Option<SLegend>,
}

#[derive(Serialize)]
struct SForeignKey {
    source_table_name: String,
    target_table_name: String,
    is_zero_one_to_one: bool,
}

impl<'a> PlantUmlDefaultGenerator<'a> {
    pub fn new() -> Result<PlantUmlDefaultGenerator<'a>, crate::SqlantError> {
        let mut str_templates = TinyTemplate::new();
        str_templates.add_template("puml", PUML_TEMPLATE)?;
        str_templates.add_template("pk", COLUMN_TEMPLATE)?;
        str_templates.add_template("ent", ENTITY_TEMPLATE)?;
        str_templates.add_template("rel", REL_TEMPLATE)?;
        str_templates.add_template("enum", ENUM_TEMPLATE)?;
        str_templates.add_template("legend", PUML_LEGEND)?;
        str_templates.set_default_formatter(&format_unescaped);
        Ok(PlantUmlDefaultGenerator { str_templates })
    }

    fn entity_render(&self, tbl: &Table) -> Result<String, crate::SqlantError> {
        enum RenderType {
            PK,     // only pk columns
            FK,     // only pure FK columns (Non PK)
            Others, // non pk and non fk
        }
        let columns_render = |rt: RenderType| -> Result<String, _> {
            Ok::<std::string::String, crate::SqlantError>(
                tbl.columns
                    .iter()
                    .filter(|col| match rt {
                        RenderType::PK => col.is_pk(),
                        RenderType::FK => !col.is_pk() && col.is_fk(),
                        RenderType::Others => !col.is_pk() && !col.is_fk(),
                    })
                    .try_fold(String::new(), |acc, col| {
                        let r = self.str_templates.render(
                            "pk",
                            &SColumn {
                                col: col.as_ref(),
                                is_fk: col.is_fk(),
                                is_pk: col.is_pk(),
                                is_nn: col.is_nn(),
                                is_nn_and_not_pk: col.is_nn() && (!col.is_pk()),
                            },
                        );
                        match r {
                            Ok(r) => Ok(acc + &r),
                            Err(e) => Err(e),
                        }
                    })?,
            )
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
}

impl<'a> ViewGenerator for PlantUmlDefaultGenerator<'a> {
    fn generate(
        &self,
        sql_erd: SqlERData,
        opts: &GeneratorConfigOptions,
    ) -> Result<String, crate::SqlantError> {
        let entities: Vec<String> = sql_erd
            .tables
            .iter()
            .map(|tbl| self.entity_render(tbl))
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
                        &SSqlEnum {
                            name: name.to_string(),
                            values: values.to_vec(),
                        },
                    )
                })
                .collect::<Result<Vec<String>, _>>()?
        } else {
            vec![]
        };

        let legend = if opts.draw_legend {
            Some(SLegend(self.str_templates.render("legend", &())?))
        } else {
            None
        };

        Ok(self.str_templates.render(
            "puml",
            &SPuml {
                entities,
                foreign_keys,
                enums,
                legend,
            },
        )?)
    }
}
