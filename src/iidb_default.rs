extern crate byteorder;

use crate::iidb::{IIDBFileOpen, IIDBFile};

use byteorder::{LittleEndian, ByteOrder};
use std::io::Read;
use std::fs::File;
use std::convert::TryFrom;
use std::collections::HashMap;

use std::convert::TryInto;

fn u(p: u32) -> usize {usize::try_from(p).unwrap()}

pub struct IIDBFileOpenDefault {
    file: IIDBFile,
    buffer: Vec<u32>
}

impl IIDBFileOpenDefault {
    pub fn new(db_file: &IIDBFile) -> IIDBFileOpenDefault {
        let mut file = File::open(&db_file.name).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let mut u32arr = vec![0; buffer.len() / 4];
        LittleEndian::read_u32_into(&buffer, &mut u32arr);

        return IIDBFileOpenDefault {
            file: db_file.clone(),
            buffer: u32arr
        };
    }
}

impl IIDBFileOpen for IIDBFileOpenDefault {
    fn get_file(&self) -> &IIDBFile {
        &self.file
    }

    fn get_as_map(&self) -> HashMap<u32, Box<[u32]>> {
        let mut map = HashMap::new();
        let range = u(self.file.range.start)..u(self.file.range.end);
        let start = range.start;
        let size = range.len();
        for i in range {
            let index = i - start;
            let offset_start = u(self.buffer[index]);
            let offset_end = if index == size-1 {self.buffer.len()} else {u(self.buffer[index+1])};
            map.insert(i.try_into().unwrap(), self.buffer[offset_start..offset_end].into());
        }
        return map;
    }

    fn get_at_pos(&self, id_: u32) -> Box<[u32]> {
        let id = u(id_);
        let range = u(self.file.range.start)..u(self.file.range.end);
        let size = range.len();
        let index = id - range.start;
        let offset_start = u(self.buffer[index]);
        let offset_end = if index == size-1 {self.buffer.len()} else {u(self.buffer[index+1])};
        return self.buffer[offset_start..offset_end].into();
    }

    fn save(&self) {
        unimplemented!()
    }
}