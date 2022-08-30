use serde::Serialize;
use std::collections::HashSet;
use std::rc::Rc;
use std::vec::Vec;

#[derive(Debug, Clone, Serialize)]
pub struct TableColumn {
    pub name: String,
    pub col_num: i16, // Can be used like id of column
    // Different SQL databases have different types.
    // Let's keep it as string not ENUM
    pub datatype: String,
    pub constraints: HashSet<ColumnConstraints>,
}

impl TableColumn {
    pub fn is_pk(&self) -> bool {
        self.constraints.contains(&ColumnConstraints::PrimaryKey)
    }
    pub fn is_fk(&self) -> bool {
        self.constraints.contains(&ColumnConstraints::ForeignKey)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
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
#[derive(Debug)]
pub struct ForeignKey {
    pub source_table: Rc<Table>,
    pub source_columns: Vec<Rc<TableColumn>>,
    pub target_table: Rc<Table>,
    pub target_columns: Vec<Rc<TableColumn>>,
    pub is_zero_one_to_one: bool, // 0..1 to 1
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Rc<TableColumn>>,
    pub has_composite_pk: bool,
}

// ERD - entity relationship diagram
pub struct SqlERData {
    pub tables: Vec<Rc<Table>>,
    pub foreign_keys: Vec<ForeignKey>,
}

pub trait SqlERDataLoader {
    // Connection string has to be passed in "constructor"
    fn load_erd_data(&mut self) -> SqlERData;
}

pub trait PlantUmlGenerator {
    fn generate(&self, sql_erd: &SqlERData) -> String;
}

impl Table {
    pub fn new(name: String, columns: Vec<Rc<TableColumn>>) -> Table {
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
        source_table: &Rc<Table>,
        source_columns: &Vec<Rc<TableColumn>>,
        target_table: &Rc<Table>,
        target_columns: &Vec<Rc<TableColumn>>,
    ) -> bool {
        if source_columns.iter().find(|col| !col.is_pk()).is_some() {
            return false;
        }
        if target_columns.iter().find(|col| !col.is_pk()).is_some() {
            return false;
        }

        // It is not is_zero_one_to_one,
        // If total count of source_table pks and target_table pks differs
        if source_table.columns.iter().filter(|c| c.is_pk()).count()
            != target_table.columns.iter().filter(|c| c.is_pk()).count()
        {
            return false;
        }

        return true;
    }

    pub fn new(
        source_table: Rc<Table>,
        source_columns: Vec<Rc<TableColumn>>,
        target_table: Rc<Table>,
        target_columns: Vec<Rc<TableColumn>>,
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
