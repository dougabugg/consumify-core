use std::{cell::RefCell, rc::Rc, collections::BTreeMap};
use std::marker::PhantomData;
use rusqlite::{Connection, Error, types::ToSql};

use crate::organize::{Media, Label, MediaLabelPair, MediaAssoc};

pub trait DbItem<T> {
    fn create(db: &Database, item: T) -> Result<Rc<RefCell<T>>, Error>;
    fn load(conn: &Connection, id: i64) -> Result<T, Error>;
    fn save(&self, conn: &Connection) -> Result<usize, Error>;
    fn cachemap_borrow_mut(map: &mut CacheMap) -> &mut CacheItem<T>;
}

pub struct DbRef<T> {
    pub id: i64,
    t: PhantomData<T>,
}

// impl DbRef<T> {
//     fn resolve(&self, db: &Database) -> Option<Rc<RefCell<T>>> {
//         db.cache.media.get(&self.id).map(|item| item.clone())
//     }
// }

impl<T> DbRef<T> where T: DbItem<T> {
    pub fn new(id: i64) -> DbRef<T> {
        DbRef::<T> {
            id: id,
            t: PhantomData
        }
    }

    pub fn resolve(&self, db: &Database) -> Option<Rc<RefCell<T>>> {
        let cache = &mut db.cache.borrow_mut();
        let map = T::cachemap_borrow_mut(cache);
        match map.get(&self.id) {
            Some(item) => Some(item.clone()),
            None => match T::load(&db.conn, self.id) {
                Ok(item) => {
                    let item = Rc::new(RefCell::new(item));
                    map.insert(self.id, item.clone());
                    Some(item)
                },
                Err(_) => None
            }
        }
    }
}

pub struct Database {
    pub conn: Connection,
    cache: RefCell<CacheMap>,
}

pub type CacheItem<T> = BTreeMap<i64, Rc<RefCell<T>>>;

pub struct CacheMap {
    pub media: CacheItem<Media>,
    pub label: CacheItem<Label>,
    pub media_label_pair: CacheItem<MediaLabelPair>,
    pub media_assoc: CacheItem<MediaAssoc>,
}