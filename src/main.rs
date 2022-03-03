use std::collections::HashMap;
use std::fs;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let mut db = match Database::new("kv.db") {
        Ok(x) => x,
        Err(e) => {
            eprintln!("kv-error: {}", e);
            return;
        }
    };

    let key = if let Some(k) = args.next() {
        k
    } else {
        eprintln!("kv-error: Expecting <key> argument");
        return;
    };

    match args.next() {
        Some(value) => db.edit(&key, &value),
        None => {
            if let Some(v) = db.view(&key) {
                println!("{}", v);
            } else {
                println!("");
                return;
            };
            return;
        }
    };

    if let Err(e) = db.save() {
        eprintln!("kv-error: {}", e);
    } else {
        println!("kv-info: Save successful")
    }
}

struct Database {
    filename: String,
    inner: HashMap<String, String>,
}

impl Database {
    /// Create a new database. Can fail on now able to read the .db file.
    pub fn new(path: &str) -> Result<Database, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let mut inner = HashMap::new();

        for line in contents.lines() {
            let chunks: Vec<&str> = line.split('\t').collect();
            if chunks.len() != 2 {
                return Err(Box::new(KvError::InvalidFormat(chunks.len()-1)));
            }
            let key = chunks[0];
            let value = chunks[1];
            inner.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            inner: inner,
            filename: path.to_owned(),
        })
    }

    /// Adds/edits a key with value taken from args.
    pub fn edit(&mut self, key: &str, value: &str) {
        self.inner.insert(key.to_owned(), value.to_owned());
    }

    /// References the value behind a requested key, if exists.
    pub fn view(&self, key: &str) -> Option<&String> {
        self.inner.get(key)
    }

    /// Takes contents of hashmap, then writes them back to disc.
    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        self.inner.iter().for_each(|(k, v)| {
            contents.push_str(&format!("{}\t{}\n", k, v))
        });

        fs::write(&self.filename, contents)
    }
}

use std::fmt::Display;
use std::fmt::Debug;
use std::error::Error;

enum KvError {
    InvalidFormat(usize),
}

impl Error for KvError {}

impl Debug for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(u) => write!(f, "Invalid kv file- found {} tab(s) on a single line (expecting 1)", u),
        }
    }
}

impl Display for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(u) => write!(f, "Invalid kv file- found {} tab(s) on a single line (expecting 1)", u),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn db_new() {
        // non-existing file
        let db = Database::new("./data/unknown.db");
        assert!(db.is_err());
        // existing file
        let db = Database::new("./data/kv-test.db");
        assert!(db.is_ok());
        // a file with invalid format
        let db = Database::new("./data/kv-invalid.db");
        assert!(db.is_err());
    }

    #[test]
    fn db_edit() {
        let mut db = Database::new("./data/kv-test.db").unwrap();
        // create new key-value pair
        db.edit("hello", "world");
        assert_eq!(db.inner.get("hello"), Some(&"world".to_owned()));
        // overwrite existing key-value pair
        db.edit("hello", "earth");
        // add another key-value pair
        db.edit("bonjour", "venus");
        assert_eq!(db.inner.get("hello"), Some(&"earth".to_owned()));
        assert_eq!(db.inner.get("bonjour"), Some(&"venus".to_owned()));
    }

    #[test]
    fn db_read() {
        let mut db = Database::new("./data/kv-test.db").unwrap();
        // create new key-value pair
        db.edit("hello", "earth");
        db.edit("bonjour", "venus");

        // existing keys
        let r = db.view("hello");
        assert_eq!(r, Some(&"earth".to_owned()));
        let r = db.view("bonjour");
        assert_eq!(r, Some(&"venus".to_owned()));
        // non-existent key
        let r = db.view("konnichiwa");
        assert_eq!(r, None);
    }

    #[test]
    fn db_save() {
        let mut db = Database::new("./data/kv-test.db").unwrap();
        // create new key-value pair
        db.edit("hello", "earth");
        db.edit("bonjour", "venus");
        assert!(db.save().is_ok());
    }

    #[test]
    fn db_reload_session() {
        let mut db = Database::new("./data/kv-test.db").unwrap();
        // create new key-value pair
        db.edit("hello", "earth");
        db.edit("bonjour", "venus");
        assert!(db.save().is_ok());

        // re-load file
        let db = Database::new("./data/kv-test.db").unwrap();
        // existing keys
        let r = db.view("hello");
        assert_eq!(r, Some(&"earth".to_owned()));
        let r = db.view("bonjour");
        assert_eq!(r, Some(&"venus".to_owned()));
    }
}