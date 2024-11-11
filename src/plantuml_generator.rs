use std::sync::Arc;

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
    {puml_lib}\n\n\
    {{ for ent in entities}}{ent}\n{{ endfor }}\n\
    {{ for fk in foreign_keys}}{fk}\n{{ endfor }}\n\
    {{ for e in enums}}{e}\n{{ endfor }}{legend}\n@enduml";

static ENTITY_TEMPLATE: &str = "table({name}) \\{\n{pks}  ---\n{fks}{nns}{others}}\n";

static COLUMN_TEMPLATE: &str = "  column({col.name}, \"{col.datatype}\"{{ if is_pk }}, $pk=true{{ endif }}{{ if is_pk }}, $fk=true{{ endif }}{{if is_nn}}, $nn=true{{ endif }})\n";

static REL_TEMPLATE: &str =
    "{source_table_name} {{ if is_zero_one_to_one }}|o--||{{else}}}o--||{{ endif }} {target_table_name}\n";

static ENUM_TEMPLATE: &str =
    "enum({name}, \"{{ for v in values}}{{if @last}}{v}{{else}}{v}, {{ endif }}{{ endfor }}\")\n";

static PUML_LEGEND: &str = r#"add_legend()"#;

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
    nns: String,    // NOT NULL Columns that don't contain both PK and FK
    others: String, // Columns that don't contain both PK and FK
}

#[derive(Serialize)]
struct SLegend(String);

#[derive(Serialize)]
struct SPuml {
    puml_lib: String,
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

struct SortedColumns {
    pks: Vec<Arc<TableColumn>>,
    fks: Vec<Arc<TableColumn>>,
    nns: Vec<Arc<TableColumn>>,
    others: Vec<Arc<TableColumn>>,
}

impl<'a> PlantUmlDefaultGenerator<'a> {
    pub fn new() -> Result<PlantUmlDefaultGenerator<'a>, crate::SqlantError> {
        let mut str_templates = TinyTemplate::new();
        str_templates.add_template("puml", PUML_TEMPLATE)?;
        str_templates.add_template("column", COLUMN_TEMPLATE)?;
        str_templates.add_template("ent", ENTITY_TEMPLATE)?;
        str_templates.add_template("rel", REL_TEMPLATE)?;
        str_templates.add_template("enum", ENUM_TEMPLATE)?;
        str_templates.add_template("legend", PUML_LEGEND)?;
        str_templates.set_default_formatter(&format_unescaped);
        Ok(PlantUmlDefaultGenerator { str_templates })
    }

    // Sorts columns in next order:
    // 1. PKs
    // 2. FKs
    // 3. NN
    // 4. Others
    fn sort_columns(cols: &[Arc<TableColumn>]) -> SortedColumns {
        let mut cloned_cols = cols.to_owned();
        let mut pks = Vec::new();
        let mut fks = Vec::new();
        let mut nns = Vec::new();
        let mut others = Vec::new();

        cloned_cols.retain(|col| {
            if col.is_pk() {
                pks.push(Arc::clone(col));
                false
            } else {
                true
            }
        });

        cloned_cols.retain(|col| {
            if col.is_fk() && !col.is_pk() {
                fks.push(Arc::clone(col));
                false
            } else {
                true
            }
        });

        cloned_cols.retain(|col| {
            if col.is_nn() && !col.is_pk() && !col.is_fk() {
                nns.push(Arc::clone(col));
                false
            } else {
                true
            }
        });

        others.extend(cloned_cols);

        SortedColumns {
            pks,
            fks,
            nns,
            others,
        }
    }
    fn entity_render(&self, tbl: &Table) -> Result<String, crate::SqlantError> {
        let sorted_columns = Self::sort_columns(&tbl.columns);

        let columns_render = |columns: Vec<Arc<TableColumn>>| -> Result<String, _> {
            Ok::<std::string::String, crate::SqlantError>(columns.iter().try_fold(
                String::new(),
                |acc, col| {
                    let r = self.str_templates.render(
                        "column",
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
                },
            )?)
        };
        Ok(self.str_templates.render(
            "ent",
            &SEntity {
                pks: columns_render(sorted_columns.pks)?,
                fks: columns_render(sorted_columns.fks)?,
                nns: columns_render(sorted_columns.nns)?,
                others: columns_render(sorted_columns.others)?,
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

        let puml_lib: String = if opts.inline_puml_lib {
            PUML_LIB_INLINE.into()
        } else {
            PUML_LIB_INCLUDE.into()
        };

        Ok(self.str_templates.render(
            "puml",
            &SPuml {
                puml_lib,
                entities,
                foreign_keys,
                enums,
                legend,
            },
        )?)
    }
}
static PUML_LIB_INCLUDE: &str = "!include https://raw.githubusercontent.com/kurotych/sqlant/b2e5db9ed8659f281208a687a344b34ff38129cd/puml-lib/db_ent.puml";

// https://raw.githubusercontent.com/kurotych/sqlant/0497c6594364e406d77dfdc0999e0b5e596b7d73/puml-lib/db_ent.puml
static PUML_LIB_INLINE: &str = r#"
!function column($name, $type, $pk=false, $fk=false, $nn=false)
  !local $prefix = ""

  !if ($pk == true)
    !$prefix = "<color:#d99d1c><&key></color>"
  !elseif($nn == true)
    !$prefix = "*"
  !endif

  !if ($fk == true)
    !$prefix = $prefix + "<color:#aaaaaa><&key></color>"
  !endif

  !return $prefix + '<b>""' + $name + '""</b>' + ': ' + '//""' + $type + '"" //'
!endfunction

!function table($name)
  !return 'entity "**' + $name + '**"' + " as " + $name
!endfunction

!procedure enum($name, $variants)
  !$list = %splitstr($variants, ",")

  object "**$name** <color:purple>**(E)**</color>" as $name {
    !foreach $item in $list
      $item
    !endfor
  }
!endprocedure

!procedure add_legend()
  legend right
   <#GhostWhite,#GhostWhite>|   |= __Legend__ |
   |<b><color:#b8861b><&key></color></b>| Primary Key |
   |<color:#aaaaaa><&key></color>| Foreign Key |
   | &#8226; | Mandatory field (Not Null) |
   | <color:purple>**(E)**</color> | Enum |
  endlegend
!endprocedure
"#;
