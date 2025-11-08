use sqlant::{lookup_loader, sql_entities::ColumnConstraints::*, sql_entities::*};
use std::{collections::BTreeMap, env};

mod utils;
use crate::utils::check_fk;

async fn load_erd() -> SqlERData {
    let con_string = env::var("CON_STRING").unwrap();
    let mut parser = lookup_loader(&con_string, "test_schema".to_string())
        .await
        .unwrap();
    parser.load_erd_data().await.unwrap()
}

#[tokio::test]
async fn custom_schema_columns() {
    let sql_er_data: SqlERData = load_erd().await;
    let tables = BTreeMap::from([
        (
            "customers",
            vec![
                ("customer_id", "integer", vec![PrimaryKey, NotNull, Unique]),
                ("customer_name", "character varying", vec![]),
            ],
        ),
        (
            "orders",
            vec![
                ("order_id", "integer", vec![PrimaryKey, NotNull, Unique]),
                ("order_description", "character varying", vec![]),
                ("customer_id", "integer", vec![ForeignKey]),
            ],
        ),
    ]);
    assert_eq!(tables.len(), sql_er_data.tables.len());
    for (table_name, cols) in tables {
        let table = sql_er_data
            .tables
            .iter()
            .find(|&t| t.name == table_name)
            .unwrap();
        assert_eq!(cols.len(), table.columns.len());

        for (exp_col_name, exp_col_type, exp_constraints) in cols {
            let col = table
                .columns
                .iter()
                .find(|col| col.name == exp_col_name)
                .unwrap();

            assert_eq!(col.constraints, exp_constraints.into_iter().collect());
            assert_eq!(col.datatype, exp_col_type);
        }
    }
}

#[tokio::test]
async fn custom_schema_fks() {
    let sql_er_data: SqlERData = load_erd().await;
    check_fk(
        &sql_er_data,
        "orders",
        "customers",
        vec!["customer_id"],
        vec!["customer_id"],
    );
}

#[tokio::test]
async fn tables_data() {
    let sql_er_data: SqlERData = load_erd().await;
    assert_eq!(sql_er_data.tables.len(), 2);
    assert_eq!(sql_er_data.foreign_keys.len(), 1);
}
