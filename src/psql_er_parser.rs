use super::sql_entities::{SqlERData, SqlERDataLoader};
use postgres::types::FromSql;
use postgres::{Client, Error, NoTls};

static GET_TABLES_LIST_QUERY: &'static str = "\
        SELECT table_name, table_name::regclass::oid as table_oid \
        FROM information_schema.tables where table_schema = 'public'";

// pg_get_constraintdef(oid) -- just for debugging purposes
static GET_FOREIGN_KEYS_QUERY: &'static str = r#"
SELECT conrelid::regclass as "source_table_name",
       confrelid::regclass as "target_table_name",
       conname AS foreign_key_name,
       oid as "foreign_key_oid",
       conkey as "source_column_nums",
       confkey as "target_columns_nums",
       conrelid, -- "source_table_oid(conrelid)",
       confrelid as "target_table_oid(confrelid)",
       pg_get_constraintdef(oid) -- just for debugging purposes
INTO TEMP TABLE temp_table
FROM   pg_constraint
WHERE  contype = 'f'
AND    connamespace = 'public'::regnamespace
ORDER  BY source_table_name;
"#;

pub struct PostgreSqlERParser {
    client: Client,
}

impl PostgreSqlERParser {
    pub fn new(connection_string: &str) -> PostgreSqlERParser {
        let mut client = Client::connect(connection_string, NoTls).unwrap();
        PostgreSqlERParser { client }
    }
}

// get list of tables and its oid
// SELECT table_name, table_name::regclass::oid as table_oid FROM information_schema.tables where
// table_schema = 'public';

impl SqlERDataLoader for PostgreSqlERParser {
    fn load_erd_data(&mut self) -> SqlERData {
        let res = &self.client.query(GET_TABLES_LIST_QUERY, &[]).unwrap();
        for row in res {
            let name: &str = row.get(0);
            let oid: u32 = row.get(1);
            println!("name: {}, oid: {}", name, oid);
        }

        let res = &self.client.query(GET_FOREIGN_KEYS_QUERY, &[]).unwrap();

        unimplemented!()
    }
}
