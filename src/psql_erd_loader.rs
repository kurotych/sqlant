use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use crate::sql_entities::ColumnConstraints;

use super::sql_entities::{ForeignKey, SqlERData, SqlERDataLoader, SqlEnums, Table, TableColumn};
use postgres::{Client, NoTls};

static GET_TABLES_LIST_QUERY: &str = r#"
SELECT trim(both '"' from table_name) as table_name
FROM information_schema.tables where table_schema = $1
"#;

/// https://www.postgresql.org/docs/current/catalog-pg-attribute.html
static GET_COLUMNS_BASIC_INFO_QUERY: &str = r#"
SELECT attname                                  AS col_name,
       attnum                                   AS col_num,
       pga.atttypid::regtype::name              AS datatype,
       attnotnull                               AS not_null,
       relname                                  AS table_name,
       pg_type.oid                              AS column_type_oid,
       typtype
FROM   pg_attribute pga
INNER JOIN pg_class
   ON pg_class.oid = pga.attrelid
INNER JOIN pg_type
   ON pga.atttypid::regtype::oid = pg_type.oid
WHERE relname =  any($1)
AND    NOT attisdropped
AND    attnum  > 0
ORDER  BY relname;
"#;

// pg_get_constraintdef(oid) -- just for debugging purposes
/// https://www.postgresql.org/docs/current/catalog-pg-constraint.html
static GET_FOREIGN_KEYS_QUERY: &str = r#"
SELECT trim(both '"' from conrelid::regclass::name)  AS source_table_name,
       trim(both '"' from confrelid::regclass::name) AS target_table_name,
       conname                   AS foreign_key_name,
       conkey                    AS source_column_nums,
       confkey                   AS target_columns_nums,
       pg_get_constraintdef(oid) -- just for debugging purposes
FROM   pg_constraint
WHERE  contype = 'f'
AND    connamespace = to_regnamespace($1)::oid
ORDER  BY source_table_name;
"#;

static GET_PKS_QUERY: &str = r#"
SELECT trim(both '"' from conrelid::regclass::name) as table_name,
       conname                  AS primary_key_name,
       conkey                   AS pk_columns_nums,
       pg_get_constraintdef(oid) -- just for debugging purposes
FROM   pg_constraint
WHERE  contype = 'p'
AND    connamespace = to_regnamespace($1)::oid
ORDER  BY table_name;
"#;

static GET_ENUM_VALUES: &str = r#"
SELECT enumlabel FROM pg_enum
JOIN pg_type ON pg_enum.enumtypid = pg_type.oid
WHERE pg_type.oid = $1;
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
    schema_name: String,
    pks: HashMap<String, Vec<i16>>,            // table_name, col_nums
    fks: HashMap<String, HashSet<FkInternal>>, // key - source_table_name
}

impl PostgreSqlERDLoader {
    pub fn new(connection_string: &str, schema_name: String) -> PostgreSqlERDLoader {
        let client = Client::connect(connection_string, NoTls).unwrap();
        PostgreSqlERDLoader {
            client,
            schema_name,
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

                    let source_columns: Vec<Rc<TableColumn>> = source_table
                        .columns
                        .iter()
                        .filter(|&col| fk.source_columns_num.contains(&col.col_num))
                        .map(Rc::clone)
                        .collect();

                    let target_table = Rc::clone(
                        tbls.iter()
                            .find(|&tbl| tbl.name == fk.target_table_name)
                            .unwrap(),
                    );

                    let target_columns: Vec<Rc<TableColumn>> = target_table
                        .columns
                        .iter()
                        .filter(|&col| fk.target_columns_num.contains(&col.col_num))
                        .map(Rc::clone)
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

    fn load_tables(&mut self, table_names: Vec<String>) -> (Vec<Rc<Table>>, SqlEnums) {
        let mut columns: HashMap<String, Vec<Rc<TableColumn>>> = HashMap::new();
        // If current database has enum types we load it here
        let mut enums: SqlEnums = HashMap::new();
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

            let typ_type: i8 = row.get("typtype");
            let tt: Result<u32, _> = typ_type.try_into();
            // enum handler
            if tt.is_ok()
                && char::from_u32(tt.unwrap()).unwrap() == 'e'
                && !enums.contains_key(col_type)
            {
                let column_type_oid: u32 = row.get("column_type_oid");
                let enum_values = self
                    .client
                    .query(GET_ENUM_VALUES, &[&column_type_oid])
                    .unwrap();

                let vals: Vec<String> = enum_values.iter().map(|v| v.get("enumlabel")).collect();
                enums.insert(col_type.to_string(), vals);
            }

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
        (
            columns
                .iter()
                .map(|(k, v)| Rc::new(Table::new(k.to_string(), v.to_vec())))
                .collect(),
            enums,
        )
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
                false
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
        for row in self
            .client
            .query(GET_PKS_QUERY, &[&self.schema_name])
            .unwrap()
        {
            self.pks
                .insert(row.get("table_name"), row.get("pk_columns_nums"));
        }
    }

    fn load_fks(&mut self) {
        let rows = self
            .client
            .query(GET_FOREIGN_KEYS_QUERY, &[&self.schema_name])
            .unwrap();
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

    fn check_is_schema_exists(&mut self) {
        let res = self
            .client
            .query(
                "SELECT EXISTS(SELECT 1 FROM pg_namespace WHERE nspname = $1)",
                &[&self.schema_name],
            )
            .unwrap();
        let row = res.first().unwrap();
        let exists: bool = row.get("exists");
        if !exists {
            panic!("Schema doesn't exist");
        }
    }
}

impl SqlERDataLoader for PostgreSqlERDLoader {
    fn load_erd_data(&mut self) -> SqlERData {
        // I use it to avoid adding schema prefixes in SQL queries
        self.client
            .query(&format!("SET search_path TO {}", self.schema_name), &[])
            .unwrap();

        self.check_is_schema_exists();

        self.load_pks();
        self.load_fks();

        let res = &self
            .client
            .query(GET_TABLES_LIST_QUERY, &[&self.schema_name])
            .unwrap();
        let table_names: Vec<String> = res.iter().map(|row| row.get("table_name")).collect();
        let (tables, enums) = self.load_tables(table_names);
        let foreign_keys = self.get_fks(&tables);

        SqlERData {
            tables,
            foreign_keys,
            enums,
        }
    }
}
