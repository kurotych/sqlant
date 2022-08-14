use sqlant::psql_er_parser::*;
use sqlant::sql_entities::*;
use std::env;

fn load_erd() -> SqlERData {
    let con_string = env::var("PSQL_CON_STRING").unwrap();
    let mut parser = PostgreSqlERParser::new(&con_string);
    parser.load_erd_data()
}

#[test]
fn tables_data() {
    let sql_er_data: SqlERData = load_erd();
    assert_eq!(sql_er_data.tables.len(), 8);
    assert_eq!(sql_er_data.foreign_keys.len(), 7);
}

#[test]
fn check_is_zero_one_to_one() {
    let sql_er_data: SqlERData = load_erd();
    for fk in &sql_er_data.foreign_keys {
        if fk.source_table.name == "vendor_address"
            || fk.source_table.name == "order_detail_approval"
        {
            assert!(fk.is_zero_one_to_one);
        } else {
            assert!(!fk.is_zero_one_to_one);
        }
    }
}

#[test]
fn composite_pk() {
    let sql_er_data: SqlERData = load_erd();
    let get_table = |table_name: &str| {
        sql_er_data
            .tables
            .iter()
            .find(|tbl| tbl.name == table_name)
            .unwrap()
    };
    for tbl in &sql_er_data.tables {
        if tbl.name == "order_detail_approval" || tbl.name == "order_detail" {
            assert!(tbl.has_composite_pk);
        } else {
            assert!(!tbl.has_composite_pk);
        }
    }
}
