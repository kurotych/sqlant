use sqlant::sql_entities::SqlERData;
pub fn check_fk(
    sql_er_data: &SqlERData,
    source_table_name: &str,
    target_table_name: &str,
    expected_source_columns: Vec<&str>,
    expected_target_columns: Vec<&str>,
) {
    let fk = sql_er_data
        .foreign_keys
        .iter()
        .find(|&fk| {
            fk.source_table.name == source_table_name && fk.target_table.name == target_table_name
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
}
