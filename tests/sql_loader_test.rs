use sqlant::{lookup_parser, sql_entities::ColumnConstraints::*, sql_entities::*};
use std::{collections::BTreeMap, env};

mod utils;
use crate::utils::check_fk;

async fn load_erd() -> SqlERData {
    let con_string = env::var("CON_STRING").unwrap();
    let mut parser = lookup_parser(&con_string, "public".to_string())
        .await
        .unwrap();
    parser.load_erd_data().await.unwrap()
}

#[tokio::test]
async fn enums() {
    let sql_er_data: SqlERData = load_erd().await;
    let mut expected_hash_map = BTreeMap::new();
    expected_hash_map.insert(
        "product_category".to_string(),
        vec![
            "electronics".to_string(),
            "jewelry".to_string(),
            "home".to_string(),
        ],
    );
    assert_eq!(sql_er_data.enums, expected_hash_map);
}

#[tokio::test]
async fn columns() {
    let sql_er_data: SqlERData = load_erd().await;
    let tables = BTreeMap::from([
        (
            "order_detail_approval",
            vec![
                (
                    "customer_order_id",
                    "bigint",
                    vec![PrimaryKey, ForeignKey, NotNull, Unique],
                ),
                (
                    "order_detail_id",
                    "bigint",
                    vec![PrimaryKey, ForeignKey, NotNull, Unique],
                ),
                ("operator_id", "bigint", vec![NotNull]),
                ("approved_at", "timestamp with time zone", vec![NotNull]),
            ],
        ),
        (
            "product",
            vec![
                ("id", "bigint", vec![PrimaryKey, NotNull, Unique]),
                ("vendor_id", "bigint", vec![ForeignKey, NotNull]),
                ("name", "text", vec![NotNull]),
                ("country", "text", vec![NotNull]),
                ("category", "product_category", vec![NotNull]),
            ],
        ),
    ]);

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
async fn fks() {
    let sql_er_data: SqlERData = load_erd().await;
    check_fk(
        &sql_er_data,
        "order_detail_approval",
        "order_detail",
        vec!["order_detail_id", "customer_order_id"],
        vec!["id", "customer_order_id"],
    );

    check_fk(
        &sql_er_data,
        "order_detail",
        "customer_order",
        vec!["customer_order_id"],
        vec!["id"],
    );
    check_fk(
        &sql_er_data,
        "order_detail",
        "sku",
        vec!["sku_id"],
        vec!["id"],
    );
    check_fk(
        &sql_er_data,
        "customer_order",
        "customer",
        vec!["customer_id"],
        vec!["id"],
    );
    check_fk(
        &sql_er_data,
        "sku",
        "product",
        vec!["product_id"],
        vec!["id"],
    );
    check_fk(
        &sql_er_data,
        "product",
        "vendor",
        vec!["vendor_id"],
        vec!["id"],
    );
    check_fk(
        &sql_er_data,
        "vendor_address",
        "vendor",
        vec!["vendor_id"],
        vec!["id"],
    );
}

#[tokio::test]
async fn tables_data() {
    let sql_er_data: SqlERData = load_erd().await;
    assert_eq!(sql_er_data.tables.len(), 8);
    assert_eq!(sql_er_data.foreign_keys.len(), 7);
}

#[tokio::test]
async fn is_zero_one_to_one() {
    let sql_er_data: SqlERData = load_erd().await;
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

#[tokio::test]
async fn composite_pk() {
    let sql_er_data: SqlERData = load_erd().await;
    for tbl in &sql_er_data.tables {
        if tbl.name == "order_detail_approval" || tbl.name == "order_detail" {
            assert!(tbl.has_composite_pk);
        } else {
            assert!(!tbl.has_composite_pk);
        }
    }
}
