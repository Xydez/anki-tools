== Anki package
An `.apkg` is a zip archive with multiple files, where `collection.anki21b` contains the collection.

The file `collection.anki21b` is a byte stream compressed using zstd which decompresses to an sqlite
database.

The sqlite database is using WAL-mode which may needs to be changed by flipping bits 18 and 19.

== Tables
The database has multiple tables.

`notes`
- `id` - note id
- `mid` - note type id
- `flds` - fields, splitted by `<US>`
- `sfld` - sorting field
