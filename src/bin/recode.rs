use std::env;
use inverted_index::iidb::*;
use inverted_index::iidb_compressed::*;
use inverted_index::iidb_default::IIDBFileOpenDefault;
use std::io;
use std::io::Write;


fn main() {
    let args: Vec<String> = env::args().collect();
    let exec_name = args.first().unwrap();
    if args.len() != 2 {
        eprintln!("usage: {} /path/to/db", exec_name);
        return
    }

    let db = IIDB::new(&args[1]);
    let files = db.get_files();
    for file in files {
        let parts: Vec<&str> = file.name.rsplitn(2, ".").collect();
        let (extension, basename) = (parts[0], parts[1]);
        match extension {
            "db2" => {
                print!("Converting: {}.db2 ... ", basename);
                io::stdout().flush().unwrap();
                let opened = IIDBFileOpenDefault::new(file);
                let nf = IIDBFileOpenCompressed::from_default(opened);
                nf.save();
                println!("done");
            }
            _ => ()
        }
    }

    return;
}