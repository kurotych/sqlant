use std::cell::{RefCell, RefMut};
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::vec::Vec;

#[derive(Debug, Clone)]
pub struct TableColumn {
    pub name: String,
    pub col_num: i16, // TODO remove me
    // Different SQL databases have different types.
    // Let's keep it as string not ENUM
    pub datatype: String,
    pub constraints: HashSet<ColumnConstraints>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnConstraints {
    NotNull,         // +
    PrimaryKey,      // +
    ForeignKey,      // + target table name
    Unique,          // +
    Check(String),   // Ex: CONSTRAINT CHK_Person CHECK (Age>=18 AND City='Sandnes') // +
    Default(String), // TODO
    Index,           // Mark NOT Unique indexes
}

#[derive(Debug)]
pub struct ForeignKey {
    pub source_table: Rc<Table>,
    pub source_columns: Vec<Rc<TableColumn>>,
    pub target_table: Rc<Table>,
    pub target_columns: Vec<Rc<TableColumn>>,
}

#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Rc<TableColumn>>, // Hashset?
}

// ERD - entity relationship diagram
pub struct SqlERData {
    pub tables: Vec<Rc<Table>>, // hashset ?
    pub foreign_keys: Vec<ForeignKey>,
}

pub trait SqlERDataLoader {
    // Connection string has to be passed in "constructor"
    fn load_erd_data(&mut self) -> SqlERData;
}
