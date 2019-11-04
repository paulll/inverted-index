use std::env;
use inverted_index::iidb::*;
use inverted_index::iidb_compressed::*;
use inverted_index::iidb_default::IIDBFileOpenDefault;
use std::io;
use std::io::Write;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let original_file = IIDBFile{
        name: String::from("/tmp/0-250.db2"),
        format: IIDBFileFormat::Default,
        range: 0..250
    };
    let compressed_file = IIDBFile{
        name: String::from("/tmp/2.iidb"),
        format: IIDBFileFormat::Compressed,
        range: 0..250
    };

    let original = original_file.open();
    let compressed = compressed_file.open();
    let original_data = original.get_as_map();
    let compressed_data = compressed.get_as_map();

    for (k,v) in original_data.iter() {
        print!("{}.. ", k);

        let c:HashSet<_> = compressed_data[k].iter().cloned().collect();
        let s:HashSet<_> = v.iter().cloned().collect();

        if c != s {
            println!("mismatch!");
            return;
        } else {
            println!("ok");
        }
    }

    //assert_eq!(original_data, compressed_data);
    return;
}