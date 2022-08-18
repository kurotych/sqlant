use sqlant::psql_er_parser::*;
use sqlant::sql_entities::*;
use std::env;

fn load_erd() -> SqlERData {
    let con_string = env::var("PSQL_CON_STRING").unwrap();
    let mut parser = PostgreSqlERParser::new(&con_string);
    parser.load_erd_data()
}

#[test]
fn test_fks() {
    let sql_er_data: SqlERData = load_erd();
    let check_fk = |source_table_name: &str,
                    target_table_name: &str,
                    expected_source_columns: Vec<&str>,
                    expected_target_columns: Vec<&str>| {
        let fk = sql_er_data
            .foreign_keys
            .iter()
            .find(|&fk| {
                fk.source_table.name == source_table_name
                    && fk.target_table.name == target_table_name
            })
            .unwrap();
        assert_eq!(fk.target_table.name, target_table_name);
        assert_eq!(expected_source_columns.len(), fk.source_columns.len());
        assert_eq!(expected_target_columns.len(), fk.target_columns.len());

        for source_column in &fk.source_columns {
            assert!(expected_source_columns.contains(&&*source_column.name));
        }
        for target_column in &fk.target_columns {
            assert!(expected_target_columns.contains(&&*target_column.name));
        }
    };
    check_fk(
        "order_detail_approval",
        "order_detail",
        vec!["order_detail_id", "customer_order_id"],
        vec!["id", "customer_order_id"],
    );

    check_fk(
        "order_detail",
        "customer_order",
        vec!["customer_order_id"],
        vec!["id"],
    );
    check_fk("order_detail", "sku", vec!["sku_id"], vec!["id"]);
    check_fk(
        "customer_order",
        "customer",
        vec!["customer_id"],
        vec!["id"],
    );
    check_fk("sku", "product", vec!["product_id"], vec!["id"]);
    check_fk("product", "vendor", vec!["vendor_id"], vec!["id"]);
    check_fk("vendor_address", "vendor", vec!["vendor_id"], vec!["id"]);
}

#[test]
fn tables_data() {
    let sql_er_data: SqlERData = load_erd();
    assert_eq!(sql_er_data.tables.len(), 8);
    assert_eq!(sql_er_data.foreign_keys.len(), 7);
}

#[test]
fn is_zero_one_to_one() {
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
    for tbl in &sql_er_data.tables {
        if tbl.name == "order_detail_approval" || tbl.name == "order_detail" {
            assert!(tbl.has_composite_pk);
        } else {
            assert!(!tbl.has_composite_pk);
        }
    }
}
