use std::vec::Vec;
struct ForeignKey<'a> {
    source_table: &'a Table<'a>,
    destination_table: &'a Table<'a>,
    source_columns: Vec<String>, // TODO?
    destionation_columns: Vec<String>,
}

enum ColumnConstraints {
    NotNull,
    Unique,
    PrimaryKey,
    ForeignKey,
    Check(String), // Ex: CONSTRAINT CHK_Person CHECK (Age>=18 AND City='Sandnes')
    Default(String),
    Index,
}

struct TableColumn {
    name: String,
    // Different SQL databases have different types.
    // Let's keep it as string not ENUM
    ctype: String,
    constraints: Vec<ColumnConstraints>,
}

struct Table<'a> {
    name: String,
    columns: Vec<TableColumn>,
    fks: Vec<ForeignKey<'a>>,
}

// ERD - entity relationship diagram

pub struct SqlERData<'a> {
    tbls: Vec<Table<'a>>,
    // Maybe some metadata will appear in future?
}

pub trait SqlERDataLoader {
    // Connection string has to be passed in "constructor"
    fn load_erd_data(&mut self) -> SqlERData;
}
