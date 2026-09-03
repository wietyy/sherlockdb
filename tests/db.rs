use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use sherlockdb::Database;

/// Each test gets its own scratch dir so parallel tests never collide.
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

struct ScratchDb {
    dir: PathBuf,
    db: Database,
}

impl ScratchDb {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "sherlockdb-test-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed),
        ));
        let db = Database::new(dir.to_str().expect("temp path must be valid UTF-8"));
        ScratchDb { dir, db }
    }
}

impl Drop for ScratchDb {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn set_then_get_roundtrips() {
    let scratch = ScratchDb::new();
    scratch.db.set("alice", "hello").expect("set failed");
    assert_eq!(scratch.db.get("alice").expect("get failed"), "hello");
}

#[test]
fn get_on_missing_key_is_not_found() {
    let scratch = ScratchDb::new();
    let err = scratch.db.get("nope").expect_err("get should fail");
    assert_eq!(err.kind(), ErrorKind::NotFound);
}

#[test]
fn set_creates_the_data_dir() {
    // Nested path that doesn't exist yet.
    let scratch = ScratchDb::new();
    let deep = scratch.dir.join("one").join("two");
    let db = Database::new(deep.to_str().expect("temp path must be valid UTF-8"));

    db.set("k", "v").expect("set failed");

    let file = deep.join("k");
    assert!(file.exists(), "data dir + file should be created");
    assert_eq!(fs::read_to_string(&file).expect("read failed"), "v");
}

#[test]
fn set_overwrites_previous_value() {
    let scratch = ScratchDb::new();
    scratch.db.set("alice", "first").expect("set failed");
    scratch.db.set("alice", "second").expect("set failed");
    assert_eq!(scratch.db.get("alice").expect("get failed"), "second");
}

#[test]
fn delete_removes_the_key() {
    let scratch = ScratchDb::new();
    scratch.db.set("alice", "hello").expect("set failed");

    scratch.db.delete("alice").expect("delete failed");

    assert!(!scratch.dir.join("alice").exists(), "file should be gone");
    let err = scratch.db.get("alice").expect_err("get should fail");
    assert_eq!(err.kind(), ErrorKind::NotFound);
}

#[test]
fn delete_is_idempotent() {
    let scratch = ScratchDb::new();
    scratch.db.delete("ghost").expect("first delete should be Ok");
    scratch.db.delete("ghost").expect("second delete should be Ok");
}

#[test]
fn keys_are_stored_separately() {
    let scratch = ScratchDb::new();
    scratch.db.set("alice", "hello").expect("set failed");
    scratch.db.set("bob", "hi").expect("set failed");

    assert_eq!(scratch.db.get("alice").expect("get failed"), "hello");
    assert_eq!(scratch.db.get("bob").expect("get failed"), "hi");
}

#[test]
fn databases_on_different_paths_are_isolated() {
    let scratch_a = ScratchDb::new();
    let scratch_b = ScratchDb::new();
    let other = Database::new(
        scratch_b
            .dir
            .to_str()
            .expect("temp path must be valid UTF-8"),
    );

    scratch_a.db.set("k", "from a").expect("set failed");

    let err = other.get("k").expect_err("other db should not see the key");
    assert_eq!(err.kind(), ErrorKind::NotFound);
}
