use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

pub struct Db {
    file: File
}

impl Db {
    pub fn open(path: &'static str) -> Result<Self, io::Error> { 
        let file_ = try!(File::create(path));

        return Ok (Db { file: file_ }); 
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<usize, io::Error> {
        let hash = djb2(key) % 1024;
        let offset = hash % 1024;

        try!(self.file.seek(SeekFrom::Start(offset)));
        let bytes_written: usize = try!(self.file.write(value));
        Ok(bytes_written)
    }

}

pub fn djb2(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for b in bytes {
        // hash * 33 + c
        hash = (hash.wrapping_shr(5) + hash) + (*b as u64);
    }

    return hash;
}