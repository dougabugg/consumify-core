use super::{Media, Label};
use std::{cell::RefCell, rc::Rc};

use crate::database::{DbItem, DbRef, Database, CacheMap, CacheItem};

pub struct Collection {
    id: String,
    columns: Vec<Column>,
    rows: Vec<Rc<RefCell<Row>>>,
}

pub struct Column {
    desc: Rc<RefCell<ColumnDesc>>,
    value: ColumnValue,
}

pub struct ColumnDesc {
    id: String,
    collection: Rc<RefCell<Collection>>,
    name: String,
}

pub enum ColumnValue {
    Text(String),
    UniqueText(Option<DbRef<UniqueText>>),
    Label(Option<DbRef<Label>>),
}

pub struct UniqueText {
    id: String,
    collection: DbRef<Collection>,
    text: String,
}

pub struct Row {
    id: String,
    collection: DbRef<Collection>,
    media: DbRef<Media>,
    columns: Vec<Column>,
}
