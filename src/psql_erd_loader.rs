use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::sql_entities::ColumnConstraints;

use super::sql_entities::{ForeignKey, SqlERData, SqlERDataLoader, Table, TableColumn};
use postgres::{Client, NoTls};

static GET_TABLES_LIST_QUERY: &'static str = r#"
SELECT trim(both '"' from table_name) as table_name
FROM information_schema.tables where table_schema = 'public'
"#;

/// https://www.postgresql.org/docs/current/catalog-pg-attribute.html
static GET_COLUMNS_BASIC_INFO_QUERY: &'static str = r#"
SELECT attname                                  AS col_name,
       attnum                                   AS col_num,
       format_type(pga.atttypid, pga.atttypmod) AS datatype,
       attnotnull                               AS not_null,
       relname                                  AS table_name
FROM   pg_attribute pga
INNER JOIN pg_class
   ON pg_class.oid = pga.attrelid
WHERE relname =  any($1)
AND    NOT attisdropped
AND    attnum  > 0
ORDER  BY relname;
"#;

// pg_get_constraintdef(oid) -- just for debugging purposes
/// https://www.postgresql.org/docs/current/catalog-pg-constraint.html
static GET_FOREIGN_KEYS_QUERY: &'static str = r#"
SELECT trim(both '"' from conrelid::regclass::name)  AS source_table_name,
       trim(both '"' from confrelid::regclass::name) AS target_table_name,
       conname                   AS foreign_key_name,
       conkey                    AS source_column_nums,
       confkey                   AS target_columns_nums,
       pg_get_constraintdef(oid) -- just for debugging purposes
FROM   pg_constraint
WHERE  contype = 'f'
AND    connamespace = 'public'::regnamespace
ORDER  BY source_table_name;
"#;

static GET_PKS_QUERY: &'static str = r#"
SELECT trim(both '"' from conrelid::regclass::name) as table_name,
       conname                  AS primary_key_name,
       conkey                   AS pk_columns_nums,
       pg_get_constraintdef(oid) -- just for debugging purposes
FROM   pg_constraint
WHERE  contype = 'p'
AND    connamespace = 'public'::regnamespace
ORDER  BY table_name;
"#;

/// https://www.postgresql.org/docs/current/view-pg-indexes.html
// If you'll need to add indexers support
// static GET_INDEXES_QUERY: &'static str = r#"
// SELECT
//     tablename,
//     indexname,
//     indexdef
// FROM
//     pg_indexes
// WHERE
//     schemaname = 'public'
// ORDER BY
//     tablename,
//     indexname;
// "#;

/// Inernal type of Foreign Key. With values that loaded from db
#[derive(Debug, Hash, PartialEq, Eq)]
struct FkInternal {
    source_table_name: String,
    source_columns_num: Vec<i16>,
    target_table_name: String,
    target_columns_num: Vec<i16>,
}

pub struct PostgreSqlERDLoader {
    client: Client,
    pks: HashMap<String, Vec<i16>>,            // table_name, col_nums
    fks: HashMap<String, HashSet<FkInternal>>, // key - source_table_name
}

impl PostgreSqlERDLoader {
    pub fn new(connection_string: &str) -> PostgreSqlERDLoader {
        let client = Client::connect(connection_string, NoTls).unwrap();
        PostgreSqlERDLoader {
            client,
            pks: HashMap::new(),
            fks: HashMap::new(),
        }
    }

    /// Return empty vector if no FKs
    fn get_fks(&self, tbls: &Vec<Rc<Table>>) -> Vec<ForeignKey> {
        let mut res = vec![];
        for tbl in tbls {
            if let Some(fks) = self.fks.get(&tbl.name) {
                for fk in fks {
                    let source_table = Rc::clone(tbl);

                    let source_columns: Vec<Rc<TableColumn>> = (&source_table.columns)
                        .into_iter()
                        .filter(|&col| fk.source_columns_num.contains(&col.col_num))
                        .map(|tc| Rc::clone(tc))
                        .collect();

                    let target_table = Rc::clone(
                        tbls.into_iter()
                            .find(|&tbl| tbl.name == fk.target_table_name)
                            .unwrap(),
                    );

                    let target_columns: Vec<Rc<TableColumn>> = (&target_table.columns)
                        .into_iter()
                        .filter(|&col| fk.target_columns_num.contains(&col.col_num))
                        .map(|tc| Rc::clone(tc))
                        .collect();

                    res.push(ForeignKey::new(
                        source_table,
                        source_columns,
                        target_table,
                        target_columns,
                    ));
                }
            }
        }
        res
    }

    fn load_tables(&mut self, table_names: Vec<String>) -> Vec<Rc<Table>> {
        let mut columns: HashMap<String, Vec<Rc<TableColumn>>> = HashMap::new();
        for tbl_name in &table_names {
            columns.insert(tbl_name.to_string(), vec![]);
        }

        let rows = self
            .client
            .query(GET_COLUMNS_BASIC_INFO_QUERY, &[&table_names])
            .unwrap();
        for row in rows {
            // I don't know how to get rid this
            let col_num: i16 = row.get("col_num");
            let col_name: &str = row.get("col_name");
            let not_null: bool = row.get("not_null");
            let tbl_name: &str = row.get("table_name");
            let col_type: &str = row.get("datatype");

            let mut constraints = self.get_constraints(tbl_name, col_num);
            if not_null {
                constraints.insert(ColumnConstraints::NotNull);
            }

            columns
                .get_mut(tbl_name)
                .unwrap()
                .push(Rc::new(TableColumn {
                    name: col_name.to_string(),
                    col_num,
                    datatype: col_type.to_string(),
                    constraints,
                }));
        }
        // Transform HashMap<String, Vec<Rc<TableColumn>>> into Vec<Table>
        columns
            .iter()
            .map(|(k, v)| Rc::new(Table::new(k.to_string(), v.to_vec())))
            .collect()
    }

    fn is_fk(&mut self, table_name: &str, table_column: i16) -> bool {
        match self.fks.get(table_name) {
            None => false,
            Some(fks) => {
                for f in fks {
                    if f.source_columns_num.contains(&table_column) {
                        return true;
                    }
                }
                return false;
            }
        }
    }

    fn is_pk(&mut self, table_name: &str, table_column: i16) -> bool {
        match self.pks.get(table_name) {
            None => false,
            Some(cols) => cols.contains(&table_column),
        }
    }

    fn get_constraints(
        &mut self,
        table_name: &str,
        table_column: i16,
    ) -> HashSet<ColumnConstraints> {
        let mut res = HashSet::new();
        if self.is_pk(table_name, table_column) {
            // The PRIMARY KEY of a table is a combination of NOT NULL and UNIQUE constraint.
            res.insert(ColumnConstraints::PrimaryKey);
            res.insert(ColumnConstraints::NotNull);
            res.insert(ColumnConstraints::Unique);
        } // todo else if NotNull and if Unique

        if self.is_fk(table_name, table_column) {
            res.insert(ColumnConstraints::ForeignKey);
        }
        // TODO other constraints
        res
    }

    fn load_pks(&mut self) {
        for row in self.client.query(GET_PKS_QUERY, &[]).unwrap() {
            self.pks
                .insert(row.get("table_name"), row.get("pk_columns_nums"));
        }
    }

    fn load_fks(&mut self) {
        let rows = self.client.query(GET_FOREIGN_KEYS_QUERY, &[]).unwrap();
        for row in rows {
            let source_table_name: String = row.get("source_table_name");
            let target_table_name: String = row.get("target_table_name");
            let source_columns_num: Vec<i16> = row.get("source_column_nums");
            let target_columns_num: Vec<i16> = row.get("target_columns_nums");

            let fk = FkInternal {
                source_table_name: source_table_name.clone(),
                source_columns_num,
                target_table_name,
                target_columns_num,
            };
            if let Some(fks) = self.fks.get_mut(&source_table_name) {
                fks.insert(fk);
            } else {
                let mut hs = HashSet::new();
                hs.insert(fk);
                self.fks.insert(source_table_name, hs);
            }
        }
    }
}

impl SqlERDataLoader for PostgreSqlERDLoader {
    fn load_erd_data(&mut self) -> SqlERData {
        self.load_pks();
        self.load_fks();

        let res = &self.client.query(GET_TABLES_LIST_QUERY, &[]).unwrap();
        let table_names: Vec<String> = res.into_iter().map(|row| row.get("table_name")).collect();
        let tables: Vec<Rc<Table>> = self.load_tables(table_names);
        let foreign_keys = self.get_fks(&tables);

        SqlERData {
            tables,
            foreign_keys,
        }
    }
}
