use crate::iidb::{IIDBFileOpen, IIDBFile};
use crate::codec::{encode_block, decode_block};

use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

use std::convert::TryFrom;
use std::convert::TryInto;
use crate::iidb_default::IIDBFileOpenDefault;
use crate::iidb::IIDBFileFormat;

pub struct IIDBFileOpenCompressed {
    file: IIDBFile,
    buffer: Vec<u8>,
    offsets: Vec<u32>,
    offsets_size: u32,
}

fn u(p: u32) -> usize {usize::try_from(p).unwrap()}

impl IIDBFileOpenCompressed {
    pub fn new(db_file: &IIDBFile) -> IIDBFileOpenCompressed {
        let mut file = File::open(&db_file.name).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        let offsets_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap()) + 4;

        return IIDBFileOpenCompressed {
            file: db_file.clone(),
            offsets: decode_block(&buffer[4..u(offsets_size)]),
            buffer,
            offsets_size
        };
    }

    pub fn from_map(db_file: &IIDBFile, map: &HashMap<u32, &[u32]>) -> IIDBFileOpenCompressed {
        let mut offsets = vec!();
        let mut data = vec!();
        let range = db_file.range.clone();
        let start = range.start;
        for id in range {
            let mut block_raw = map[&id].to_vec();
            offsets.push(u32::try_from(data.len()).unwrap());
            block_raw.sort();
            data.append(&mut encode_block(&block_raw));
        }
        let mut offsets_encoded = encode_block(&offsets);
        let offsets_size = u32::try_from(offsets_encoded.len()).unwrap();
        let mut buffer = vec!(0u8;4);

        buffer.copy_from_slice(&offsets_size.to_le_bytes());
        buffer.append(&mut offsets_encoded);
        buffer.append(&mut data);

        return IIDBFileOpenCompressed {
            file: db_file.clone(),
            buffer,
            offsets,
            offsets_size
        };
    }

    pub fn from_default(source: IIDBFileOpenDefault) -> IIDBFileOpenCompressed {
        let source_file = source.get_file();
        let range = source_file.range.clone();

        let mut offsets = vec!();
        let mut data = vec!();
        let start = range.start;
        for id in range {
            let mut block_raw = source.get_at_pos(id);
            block_raw.sort();
            offsets.push(u32::try_from(data.len()).unwrap());
            data.append(&mut encode_block(&block_raw));
        }
        let mut offsets_encoded = encode_block(&offsets);
        let offsets_size = u32::try_from(offsets_encoded.len()).unwrap();
        let mut buffer = vec!(0u8;4);

        buffer.copy_from_slice(&offsets_size.to_le_bytes());
        buffer.append(&mut offsets_encoded);
        buffer.append(&mut data);

        let source_file_basename = &source_file.name[..source_file.name.len()-4];
        return IIDBFileOpenCompressed {
            file: IIDBFile {
                name: format!("{}.iidb", source_file_basename),
                format: IIDBFileFormat::Compressed,
                range: source_file.range.clone()
            },
            buffer,
            offsets,
            offsets_size
        };
    }
}

impl IIDBFileOpen for IIDBFileOpenCompressed {
    fn get_file(&self) -> &IIDBFile {
        return &self.file
    }

    fn get_as_map(&self) -> HashMap<u32, Box<[u32]>> {
        let mut map = HashMap::new();
        let range = self.file.range.clone();
        for id in range {
            map.insert(id, self.get_at_pos(id));
        }
        return map;
    }

    fn get_at_pos(&self, id: u32) -> Box<[u32]> {
        let index = id - self.file.range.start;
        let offset_start = self.offsets[u(index)] + self.offsets_size;
        let offset_end = if index != self.file.range.end - 1 {
            u(self.offsets[u(index+1)] + self.offsets_size)
        } else {
            self.buffer.len()
        };
        return decode_block(&self.buffer[u(offset_start)..offset_end]).into_boxed_slice();
    }

    fn save(&self) {
        let mut file = File::create(&self.file.name).unwrap();
        file.write_all(&self.buffer);
        file.sync_data();
    }
}