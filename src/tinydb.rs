use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
extern crate bincode;


pub struct Db<'a> {
    file: File,
    btree: BTreeMap<&'a [u8], &'a [u8]>,
}

impl<'a> Db<'a> {
    pub fn open(path: &'static str) -> Result<Self, io::Error> {
        let file_ = try!(File::create(path));
        
        Ok(Db {
            file: file_,
            btree: BTreeMap::new(),
        })
    }

    pub fn put(&mut self, key: &'a[u8], value: &'a[u8]) -> Result<usize, io::Error> {
        self.btree.insert(key, value);
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

#[cfg(test)]
mod tests {
    extern crate bincode;
    use tinydb;
    use std::collections::BTreeMap;

    #[test]
    fn test_open_ok() {
        let d = tinydb::Db::open("test.db");
        assert_eq!(d.is_ok(), true);
    }

    #[test]
    fn test_put_ok() {
        let mut d = tinydb::Db::open("test.db").unwrap();
        let w = d.put("a".as_bytes(), "b".as_bytes());
        assert_eq!(w.is_ok(), true);
    }

    #[test]
    fn test_serd_serialize() {
        let mut btree: BTreeMap<Vec<u8>, Vec<u8>> = BTreeMap::new();
        btree.insert("a".as_bytes().to_vec(), "b".as_bytes().to_vec());

        let limit = bincode::SizeLimit::Bounded(1000000);
        let s: Vec<u8> = bincode::serde::serialize(&btree, limit).unwrap();
        let new_map: BTreeMap<Vec<u8>, Vec<u8>> = bincode::serde::deserialize(&s).unwrap();

        let key = "a".as_bytes().to_vec();
        let val = "b".as_bytes().to_vec();
        assert_eq!(new_map.get(&key), Some(&val));
    }

    // #[test]
    // fn test_put_and_get(){
    //     let mut d = tinydb::Db::open("test1.db").unwrap();
    //     let w = d.put("a".as_bytes(), "b".as_bytes());
    //     let res = d.get("a".as_bytes()).unwrap();
    //     assert_eq!(res.is_some(), true);
    // }
}
