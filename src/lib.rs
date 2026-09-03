//! A minimal, zero-dependency, file-backed key-value store.
//!
//! Everything lives in one directory: each key you [`set`](Database::set) is
//! stored as its own plain file, named after the key. No hidden state, no
//! dependencies, no magic — you can open the files and read them with any
//! editor.
//!
//! # Example
//!
//! ```
//! use sherlockdb::Database;
//! use std::fs;
//!
//! let dir = std::env::temp_dir().join(format!("sherlockdb-docs-{}", std::process::id()));
//! let _ = fs::remove_dir_all(&dir); // clean slate
//! let db = Database::new(dir.to_str().unwrap());
//!
//! db.set("alice", "hello").unwrap();
//! assert_eq!(db.get("alice").unwrap(), "hello");
//!
//! db.set("bob", "hi").unwrap();
//! db.delete("alice").unwrap(); // idempotent — deleting a
//! db.delete("alice").unwrap(); // missing key is also fine
//!
//! assert!(db.get("alice").is_err()); // NotFound
//! assert_eq!(db.get("bob").unwrap(), "hi");
//!
//! fs::remove_dir_all(&dir).unwrap(); // tidy up the example
//! ```
//!
//! # Errors
//!
//! Operations return [`std::io::Error`]. Missing keys surface as
//! [`ErrorKind::NotFound`](std::io::ErrorKind::NotFound) — except for
//! [`delete`](Database::delete), which treats a missing key as success.

#![warn(missing_docs)]

use std::fs;
use std::path::PathBuf;

/// A key-value store persisted as plain files inside a single directory.
///
/// Construct one with [`Database::new`]; the directory isn't touched until
/// the first [`set`](Database::set), which creates it on demand.
///
/// The struct carries no state of its own beyond the directory path — every
/// call reads or writes the filesystem directly, so multiple `Database`
/// instances pointing at the same path see the same data.
pub struct Database {
    path: PathBuf,
}

impl Database {
    /// Opens a store rooted at `path`.
    ///
    /// Nothing is created or modified yet — the directory is made lazily on
    /// the first [`set`](Database::set).
    pub fn new(path: &str) -> Self {
        Database {
            path: PathBuf::from(path),
        }
    }

    /// Stores `contents` under `id`.
    ///
    /// Creates the data directory (and any missing parents) if it doesn't
    /// exist, then writes the value to a single file at `{path}/{id}`.
    /// Setting an existing key overwrites its previous value.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory can't be created or the file can't
    /// be written.
    pub fn set(&self, id: &str, contents: &str) -> Result<(), std::io::Error> {
        let filepath: PathBuf = self.path.join(id);
        fs::create_dir_all(&self.path)?;
        fs::write(filepath, contents)
    }

    /// Reads back the value stored under `id`.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::NotFound`](std::io::ErrorKind::NotFound) if `id`
    /// has never been set, and other [`std::io::Error`]s for genuine
    /// filesystem failures (permissions, etc.).
    pub fn get(&self, id: &str) -> Result<String, std::io::Error> {
        let filepath = self.path.join(id);
        fs::read_to_string(filepath)
    }

    /// Removes the key `id` and its file.
    ///
    /// Idempotent: deleting a key that doesn't exist still returns `Ok`, so
    /// there's no need to check before deleting.
    ///
    /// # Errors
    ///
    /// Returns an error only if the filesystem itself fails, such as a
    /// permission problem.
    pub fn delete(&self, id: &str) -> Result<(), std::io::Error> {
        let filepath = self.path.join(id);
        match fs::remove_file(filepath) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e),
        }
    }
}
