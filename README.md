# sherlockdb

A minimal, zero-dependency, file-backed key-value store for Rust.

Each key is stored as one plain file inside a data directory — no hidden state, no magic.

## Usage

```toml
[dependencies]
sherlockdb = "0.1"
```

```rust
use sherlockdb::Database;

let db = Database::new("data");

db.set("user:1", "alice").unwrap();
assert_eq!(db.get("user:1").unwrap(), "alice");

db.delete("user:1").unwrap();
```

## API

- `Database::new(path)` — open (or create on first write) a data directory
- `set(id, contents)` — write a key
- `get(id)` — read a key; `Err(NotFound)` if it doesn't exist
- `delete(id)` — remove a key; idempotent

## License

MIT
