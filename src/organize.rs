pub mod collection;

use std::{cell::RefCell, rc::Rc};
use rusqlite::{Connection, Error, types::{ToSql, FromSql}};

use crate::database::{DbItem, DbRef, Database, CacheMap, CacheItem};

pub struct Media {
    id: i64,
    name: String,
}

impl DbItem<Media> for Media {
    fn create(db: &Database, mut item: Media) -> Result<Rc<RefCell<Media>>, Error> {
        let result = db.conn.execute(
            "INSERT INTO media (name) VALUES (?)",
            &[&item.name]
        );
        item.id = db.conn.last_insert_rowid();
        result.map(|_| Rc::new(RefCell::new(item)))
    }

    fn load(conn: &Connection, id: i64) -> Result<Media, Error> {
        conn.query_row(
            "SELECT name FROM media WHERE id = ?",
            &[id],
            |row| Media {
                id: id,
                name: row.get(0),
            }
        )
    }

    fn save(&self, conn: &Connection) -> Result<usize, Error> {
        conn.execute(
            "UPDATE media SET name = ? WHERE id = ?",
            &[
                &self.name as &dyn ToSql,
                &self.id
            ]
        )
    }

    fn cachemap_borrow_mut(map: &mut CacheMap) -> &mut CacheItem<Media> {
        &mut map.media
    }
}

pub struct Label {
    id: i64,
    parent: Option<DbRef<Media>>,
    name: String,
    position: i64,
}

impl DbItem<Label> for Label {
    fn create(db: &Database, mut item: Label) -> Result<Rc<RefCell<Label>>, Error> {
        let result = db.conn.execute(
            "INSERT INTO label (parent, name, position) VALUE (?, ?, ?)",
            &[
                &match &item.parent {
                    Some(r) => r.id,
                    None => -1,
                } as &dyn ToSql,
                &item.name,
                &item.position,
            ]
        );
        item.id = db.conn.last_insert_rowid();
        result.map(|_| Rc::new(RefCell::new(item)))
    }

    fn load(conn: &Connection, id: i64) -> Result<Label, Error> {
        conn.query_row(
            "SELECT parent, name, position FROM label WHERE id = ?",
            &[id],
            |row| Label {
                id: id,
                parent: match row.get(0) {
                    -1 => None,
                    p => Some(DbRef::new(p))
                },
                name: row.get(1),
                position: row.get(2)
            }
        )
    }

    fn save(&self, conn: &Connection) -> Result<usize, Error> {
        conn.execute(
            "UPDATE label SET parent = ?, name = ?, position = ? WHERE id = ?",
            &[
                &match &self.parent {
                    Some(dbref) => dbref.id,
                    None => -1
                } as &dyn ToSql,
                &self.name,
                &self.position,
                &self.id
            ]
        )
    }

    fn cachemap_borrow_mut(map: &mut CacheMap) -> &mut CacheItem<Label> {
        &mut map.label
    }
}

pub struct MediaLabelPair {
    id: i64,
    media: DbRef<Media>,
    label: DbRef<Label>,
    position: i64,
}

impl DbItem<MediaLabelPair> for MediaLabelPair {
    fn create(db: &Database, mut item: MediaLabelPair) -> Result<Rc<RefCell<MediaLabelPair>>, Error> {
        let result = db.conn.execute(
            "INSERT INTO media_label_pair (media, label, position) VALUES (?, ?, ?)",
            &[
                &item.media.id as &dyn ToSql,
                &item.label.id,
                &item.position
            ]
        );
        item.id = db.conn.last_insert_rowid();
        result.map(|_| Rc::new(RefCell::new(item)))
    }

    fn load(conn: &Connection, id: i64) -> Result<MediaLabelPair, Error> {
        conn.query_row(
            "SELECT media, label, position FROM media_label_pair WHERE id = ?",
            &[id],
            |row| MediaLabelPair {
                id: id,
                media: DbRef::new(row.get(0)),
                label: DbRef::new(row.get(1)),
                position: row.get(2)
            }
        )
    }

    fn save(&self, conn: &Connection) -> Result<usize, Error> {
        conn.execute(
            "UPDATE media_label_pair SET media = ?, label = ?, position = ? WHERE id = ?",
            &[
                &self.media.id,
                &self.label.id,
                &self.position,
                &self.id
            ]
        )
    }

    fn cachemap_borrow_mut(map: &mut CacheMap) -> &mut CacheItem<MediaLabelPair> {
        &mut map.media_label_pair
    }
}

pub struct MediaAssoc {
    id: i64,
    media_a: DbRef<Media>,
    media_b: DbRef<Media>,
    kind: AssocKind,
    position: i64,
}

pub enum AssocKind {
    PreviewImage,
    DescText,
    CustomAssoc(String),
}

impl AssocKind {
    fn to_sql_str(&self) -> &str {
        match self {
            AssocKind::PreviewImage => "$preview-image",
            AssocKind::DescText => "$desc-text",
            AssocKind::CustomAssoc(s) => &s,
        }
    }

    fn from_sql_str(s: String) -> Self {
        match s.as_str() {
            "$preview-image" => AssocKind::PreviewImage,
            "$desc-text" => AssocKind::DescText,
            _ => AssocKind::CustomAssoc(s)
        }
    }
}

impl DbItem<MediaAssoc> for MediaAssoc {
    fn create(db: &Database, mut item: MediaAssoc) -> Result<Rc<RefCell<MediaAssoc>>, Error> {
        let result = db.conn.execute(
            "INSERT INTO media_assoc (media_a, media_b, kind, position) VALUES (?, ?, ?, ?)",
            &[
                &item.media_a.id as &dyn ToSql,
                &item.media_b.id,
                &item.kind.to_sql_str()
            ]
        );
        item.id = db.conn.last_insert_rowid();
        result.map(|_| Rc::new(RefCell::new(item)))
    }

    fn load(conn: &Connection, id: i64) -> Result<MediaAssoc, Error> {
        conn.query_row(
            "SELECT media_a, media_b, kind, position FROM media_assoc WHERE id = ?",
            &[id],
            |row| MediaAssoc {
                id: id,
                media_a: DbRef::new(row.get(0)),
                media_b: DbRef::new(row.get(1)),
                kind: AssocKind::from_sql_str(row.get(2)),
                position: row.get(3)
            }
        )
    }

    fn save(&self, conn: &Connection) -> Result<usize, Error> {
        conn.execute(
            "UPDATE media_assoc SET media_a = ?, media_b = ?, kind = ?, position = ? WHERE id = ?",
            &[
                &self.media_a.id as &dyn ToSql,
                &self.media_b.id,
                &self.kind.to_sql_str(),
                &self.position,
            ]
        )
    }

    fn cachemap_borrow_mut(map: &mut CacheMap) -> &mut CacheItem<MediaAssoc> {
        &mut map.media_assoc
    }
}
