extern crate itoa;
extern crate byteorder;

use inverted_index::iidb::*;

use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let exec_name = args.first().unwrap();
    if args.len() != 3 {
        eprintln!("usage: {} /path/to/db id", exec_name);
        return
    }

    let id = match args.last().unwrap().parse::<u32>() {
        Err(_) => {
            eprintln!("usage: {} /path/to/db id", exec_name);
            return
        }
        Ok(v) => v
    };

    let db = IIDB::new(&args[1]);
    db.query(id, |result| {
        match result {
            Some(v) => {
                let stdout = io::stdout();
                let mut lock = stdout.lock();
                for &x in v.into_iter() {
                    itoa::write(&mut lock, x);
                    lock.write_all(b" ");
                }
            },
            None => {
                eprintln!("not found");
            }
        }
    });

    return;
}