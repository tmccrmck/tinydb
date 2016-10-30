use std::io;
use std::io::prelude::*;
extern crate bincode;

use std::fs::File;
use std::collections::BTreeMap;


pub struct Db {
    file: File,
    btree: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Db {
    pub fn open(path: &'static str) -> Result<Self, io::Error> {
        let file_ = try!(File::create(path));
        
        Ok(Db {
            file: file_,
            btree: BTreeMap::new(),
        })
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<usize, io::Error> {
        self.btree.insert(key.to_vec(), value.to_vec());
        let limit = bincode::SizeLimit::Bounded(1000000);
        let s: Vec<u8> = bincode::serde::serialize(&self.btree, limit).unwrap();
        return self.file.write(&s);
    }

    pub fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, io::Error> {
        let mut buf = Vec::new();
        try!(self.file.read_to_end(&mut buf));
        let map: BTreeMap<Vec<u8>, Vec<u8>> = bincode::serde::deserialize(&buf).unwrap();
        let rtn: Option<&Vec<u8>> = map.get(&key.to_vec());
        match rtn {
            None => return Ok(None),
            Some(x) => return Ok(Some(x.to_owned())),
        }
    }
}
