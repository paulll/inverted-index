use std::fs;
use std::ops::Range;
use std::collections::HashMap;
use crate::iidb_default::IIDBFileOpenDefault;
use crate::iidb_compressed::IIDBFileOpenCompressed;
use std::cmp::Ordering::*;




#[derive(Clone, Debug)]
pub enum IIDBFileFormat {
    Default,
    Compressed
}

pub trait IIDBFileOpen {
    fn get_file(&self) -> &IIDBFile;
    fn get_as_map(&self) -> HashMap<u32, Box<[u32]>>;
    fn get_at_pos(&self, id: u32) -> Box<[u32]>;

    fn save(&self);
}

#[derive(Clone, Debug)]
pub struct IIDBFile {
    pub name: String,
    pub format: IIDBFileFormat,
    pub range: Range<u32>
}

pub struct IIDB {
    path: String,
    files: Vec<IIDBFile>
}

impl IIDB {
    pub fn new(path: &str) -> IIDB {
        let mut db = IIDB {
            path: path.to_string(),
            files: vec!()
        };
        db.update_index();
        return db;
    }

    fn update_index(&mut self) {
        self.files.clear();
        let paths = fs::read_dir(&self.path).unwrap();
        for path in paths {
            let pathbuf = path.unwrap().path();
            let basename = pathbuf.file_name().unwrap().to_str().unwrap();
            let ext_split: Vec<&str> = basename.split(".").collect();
            if ext_split.len() == 1 {continue}
            let (rng, ext) = (ext_split[0], ext_split[1]);
            match ext {
                "db2" => {
                    let parts: Vec<&str> = rng.split("-").collect();
                    let (from, to): (u32, u32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap());
                    self.files.push(IIDBFile {
                        name: pathbuf.to_str().unwrap().to_string(),
                        range: from..to,
                        format: IIDBFileFormat::Default
                    });
                }
                "iidb" => {
                    let parts: Vec<&str> = rng.split("-").collect();
                    let (from, to): (u32, u32) = (parts[0].parse().unwrap(), parts[1].parse().unwrap());
                    self.files.push(IIDBFile {
                        name: pathbuf.to_str().unwrap().to_string(),
                        range: from..to,
                        format: IIDBFileFormat::Compressed
                    });
                }
                _ => ()
            }
        }
        self.files.sort_unstable_by_key(|v| v.range.start);
    }

    pub fn lookup_file(&self, id: u32) -> Option<&IIDBFile> {
        return match self.files.binary_search_by(|file| {
            if id < file.range.start {
                Greater
            } else if id >= file.range.end {
                Less
            } else {
                Equal
            }
        }) {
            Ok(idx) => Some(&self.files[idx]),
            Err(_) => None
        }
    }

    pub fn query<F>(&self, id: u32, callback: F) -> ()
        where F: Fn(Option<Box<[u32]>>)->()
    {
        let file = match self.lookup_file(id) {
            Some(v) => v,
            None => return callback(None)
        };
        let fd = file.open();
        let data = Some(fd.get_at_pos(id));
        callback(data);
    }

    pub fn get_files(&self) -> &[IIDBFile]{
        return &self.files;
    }
}

impl IIDBFile {
    pub fn open(&self) -> Box<dyn IIDBFileOpen> {
        return match self.format {
            IIDBFileFormat::Compressed => Box::new(IIDBFileOpenCompressed::new(&self)),
            IIDBFileFormat::Default => Box::new(IIDBFileOpenDefault::new(&self)),
            _ => unimplemented!()
        }
    }
}