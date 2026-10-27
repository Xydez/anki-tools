use std::{borrow::Cow, collections::HashMap};

use unicase::UniCase;

pub struct Database {
    conn: rusqlite::Connection,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Note {
    pub id: i64,
    pub ntid: i64,
    pub fields: Vec<String>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct NoteType {
    pub ntid: i64,
    pub name: Option<String>,
    pub fields: Vec<String>,
}

impl Database {
    pub fn open_serialized(mut data: Vec<u8>) -> rusqlite::Result<Database> {
        // Force the database into rollback mode as it may not be opened in WAL-mode.
        //
        //   The application can set the file format version numbers (bytes 18 and 19) of the
        //   input database P to 0x01 prior to invoking sqlite3_deserialize(D,S,P,N,M,F) to
        //   force the database file into rollback mode and work around this limitation.
        //
        //   https://sqlite.org/c3ref/deserialize.html

        data[18] = 0x01;
        data[19] = 0x01;

        let mut conn = rusqlite::Connection::open_in_memory()?;

        conn.deserialize_read_exact(
            rusqlite::MAIN_DB,
            std::io::Cursor::new(&data),
            data.len(),
            true
        )?;

        // In order to perform certain queries on the database we need to handle the "unicase"
        // collation used by some tables. Thankfully, the exact same library used by Anki may be
        // used.
        conn.create_collation(
            "unicase",
            |a, b| UniCase::new(a).cmp(&UniCase::new(b))
        )?;

        Ok(Database { conn })
    }

    fn fetch_note_types(&self) -> rusqlite::Result<HashMap<i64, String>> {
        self.conn
            .prepare("SELECT id, name FROM notetypes")?
            .query_map([], |row| {
                let ntid: i64 = row.get(0)?;
                let name: String = row.get(1)?;

                Ok((ntid, name))
            })?
            .collect()
    }

    fn fetch_note_type_fields(&self) -> rusqlite::Result<Vec<(i64, String)>> {
        self.conn
            .prepare("SELECT ntid, name FROM fields ORDER BY ntid, ord")?
            .query_map([], |row| {
                let ntid: i64 = row.get(0)?;
                let name: String = row.get(1)?;

                Ok((ntid, name))
            })?
            .collect()
    }

    pub fn get_note_types(&self) -> rusqlite::Result<HashMap<i64, NoteType>> {
        let notetype_names = self.fetch_note_types()?;
        //dbg!(&notetype_names);

        let fields = self.fetch_note_type_fields()?;
        //dbg!(&fields);

        let mut notetypes = notetype_names
            .into_iter()
            .map(|(ntid, ntname)| {
                let nt = NoteType {
                    ntid,
                    name: Some(ntname),
                    fields: Vec::new()
                };

                (ntid, nt)
            })
            .collect::<HashMap<_, _>>();

        for (ntid, fname) in fields {
            notetypes
                .entry(ntid)
                .or_insert_with(|| NoteType { ntid, name: None, fields: Vec::new() })
                .fields
                .push(fname);
        }

        Ok(notetypes)
    }

    pub fn get_notes(&self) -> rusqlite::Result<HashMap<i64, Note>> {
        let mut query = self.conn.prepare("SELECT id, mid, flds FROM notes")?;

        query
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let ntid: i64 = row.get(1)?;
                let flds: String = row.get(2)?;

                const FIELD_SEPARATOR: char = '\u{001f}';

                let fields = flds
                    .split(FIELD_SEPARATOR)
                    .map(ToOwned::to_owned)
                    .collect();

                let note = Note {
                    id,
                    ntid,
                    fields,
                };

                Ok((id, note))
            })?
            .collect()
    }
}

#[allow(unused)]
pub fn val_to_str<'a>(value_ref: rusqlite::types::ValueRef<'a>) -> Cow<'a, str> {
    match value_ref {
        rusqlite::types::ValueRef::Null => Cow::from("null"),
        rusqlite::types::ValueRef::Integer(i) => Cow::from(i.to_string()),
        rusqlite::types::ValueRef::Real(i) => Cow::from(i.to_string()),
        rusqlite::types::ValueRef::Text(x) => Cow::from(str::from_utf8(x).unwrap()),
        rusqlite::types::ValueRef::Blob(x) => Cow::from(format!("{x:?}"))
    }
}
