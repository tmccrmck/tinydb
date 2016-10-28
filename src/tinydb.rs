use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;

pub struct Db<'a> {
    file: File,
    btree: BTreeMap<&'a [u8], &'a [u8]>,
}

impl<'a> Db<'a> {
    pub fn open(path: &'static str) -> Result<Self, io::Error> {
        let file_ = try!(File::create(path));

        return Ok(Db { file: file_, btree: BTreeMap::new() });
    }

    pub fn put(&mut self, key: &'a [u8], value: &'a [u8]) -> Option<&[u8]> {
        self.btree.insert(key, value)
    }
}
