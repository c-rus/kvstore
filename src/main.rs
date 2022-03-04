//! Crate    : kvstore
//! Author   : Chase Ruskin
//! File     : main.rs
//! Abstract : 
//!     Entry-point to `kvstore` command-line tool. The main process follows
//!         1. reads env and accept arguments, 
//!         2. loads database from a file, 
//!         3. interacts with database
//!         4. Saves any necessary changes to database.

use kvstore::database::*;
use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let root = env::var("KVSTORE_HOME").unwrap_or(".".to_owned());
    let mut db = match Database::new(&(root+"/kv.db")) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("kv-error: {}", e);
            return;
        }
    };

    let key = if let Some(k) = args.next() {
        k
    } else {
        println!("{}", USAGE);
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

const USAGE: &str = "\
kvstore is a key-value keeper.

Usage:
    kvstore [<key>] [<value>]

Args:
    <key>       label to identify data
    <value>     data to store behind a label

More:
    kvstore's database is a 'kv.db' file located where the program is ran
    unless the environment variable KVSTORE_HOME is set to an existing 
    directory.";