use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::vec::Vec;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TableColumn {
    pub name: String,
    pub col_num: i16, // Can be used like id of column
    // Different SQL databases have different types.
    // Let's keep it as string not ENUM
    pub datatype: String,
    pub constraints: BTreeSet<ColumnConstraints>,
}

impl TableColumn {
    pub fn is_pk(&self) -> bool {
        self.constraints.contains(&ColumnConstraints::PrimaryKey)
    }
    pub fn is_fk(&self) -> bool {
        self.constraints.contains(&ColumnConstraints::ForeignKey)
    }
    pub fn is_nn(&self) -> bool {
        self.constraints.contains(&ColumnConstraints::NotNull)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ColumnConstraints {
    NotNull,
    PrimaryKey,
    ForeignKey,
    Unique,
    Check(String), // TODO Ex: CONSTRAINT CHK_Person CHECK (Age>=18 AND City='Sandnes') // +
    Default(String), // TODO
    Index,         // Mark NOT Unique indexes
}

// Types of relationship https://launchschool.com/books/sql_first_edition/read/multi_tables
#[derive(Clone, Debug)]
pub struct ForeignKey {
    pub source_table: Arc<Table>,
    pub source_columns: Vec<Arc<TableColumn>>,
    pub target_table: Arc<Table>,
    pub target_columns: Vec<Arc<TableColumn>>,
    pub is_zero_one_to_one: bool, // 0..1 to 1
}

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Arc<TableColumn>>,
    pub has_composite_pk: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct View {
    pub materialized: bool,
    pub name: String,
    pub columns: Vec<Arc<TableColumn>>,
}

// key - enum_name (type) v = enum values
pub type SqlEnums = BTreeMap<String, Vec<String>>;

// ERD - entity relationship diagram
#[derive(Clone, Debug)]
pub struct SqlERData {
    pub tables: Vec<Arc<Table>>,
    pub foreign_keys: Vec<ForeignKey>,
    pub enums: SqlEnums,
    pub views: Vec<Arc<View>>,
}

#[async_trait::async_trait]
pub trait SqlERDataLoader {
    // Connection string has to be passed in "constructor"
    async fn load_erd_data(&mut self) -> Result<SqlERData, crate::SqlantError>;
}

impl Table {
    pub fn new(name: String, columns: Vec<Arc<TableColumn>>) -> Table {
        let has_composite_pk = columns.iter().fold(0, |acc, x| {
            acc + x // there is a bit overhead here, because we can interrupt after acc > 1
                .as_ref() // but it looks nicely than `for ... in ...` loop
                .constraints
                .contains(&ColumnConstraints::PrimaryKey) as u16
        }) > 1; // if more than 1 pk, it is composite

        Table {
            name,
            columns,
            has_composite_pk,
        }
    }
}

impl ForeignKey {
    fn is_zero_one_to_one(
        source_table: &Arc<Table>,
        source_columns: &[Arc<TableColumn>],
        target_table: &Arc<Table>,
        target_columns: &[Arc<TableColumn>],
    ) -> bool {
        if source_columns.iter().any(|col| !col.is_pk()) {
            return false;
        }
        if target_columns.iter().any(|col| !col.is_pk()) {
            return false;
        }

        // It is not is_zero_one_to_one,
        // If total count of source_table pks and target_table pks differs
        if source_table.columns.iter().filter(|c| c.is_pk()).count()
            != target_table.columns.iter().filter(|c| c.is_pk()).count()
        {
            return false;
        }

        true
    }

    pub fn new(
        source_table: Arc<Table>,
        source_columns: Vec<Arc<TableColumn>>,
        target_table: Arc<Table>,
        target_columns: Vec<Arc<TableColumn>>,
    ) -> ForeignKey {
        let is_zero_one_to_one = Self::is_zero_one_to_one(
            &source_table,
            &source_columns,
            &target_table,
            &target_columns,
        );
        ForeignKey {
            source_table,
            source_columns,
            target_table,
            target_columns,
            is_zero_one_to_one,
        }
    }
}
