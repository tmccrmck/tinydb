extern crate bincode;

use std::io;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::collections::HashMap;


pub struct Db<'a> {
    dir: PathBuf,
    btree: BTreeMap<&'a [u8], &'a [u8]>,
    offsets: HashMap<&'a [u8], i32>,
}

impl<'a> Db<'a> {
    pub fn open(dir: PathBuf) -> Result<Self, io::Error> {
        //let file_ = try!(File::create(path));
        
        Ok(Db {
            dir: dir,
            btree: BTreeMap::new(),
            offsets: HashMap::new(),
        })
    }

    pub fn put(&mut self, key: &'a [u8], value: &'a [u8]) -> Result<usize, io::Error> {
        self.btree.insert(key, value);
        return Ok(1); // placeholder
        //let limit = bincode::SizeLimit::Bounded(1000000);
        //let s: Vec<u8> = bincode::serde::serialize(&self.btree, limit).unwrap();
        //return self.file.write(&s);
    }

    pub fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, io::Error> {
        let x = self.btree.get(key);
        match x {
            Some(val) => Ok(Some(val.to_vec())),
            None => Ok(None),
        }
        // let mut buf = Vec::new();
        // try!(self.file.read_to_end(&mut buf));
        // let map: BTreeMap<Vec<u8>, Vec<u8>> = bincode::serde::deserialize(&buf).unwrap();
        // let rtn: Option<&Vec<u8>> = map.get(&key.to_vec());
        // match rtn {
        //     None => return Ok(None),
        //     Some(x) => return Ok(Some(x.to_owned())),
        // }
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.btree.remove(key);
    }
}

#[cfg(test)]
mod tests {
    extern crate bincode;
    extern crate tempdir;

    use tinydb;
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn test_open_ok() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let d = tinydb::Db::open(tmp_dir.into_path());
        assert_eq!(d.is_ok(), true);
    }

    #[test]
    fn test_put_ok() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut d = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        let w = d.put("a".as_bytes(), "b".as_bytes());
        assert_eq!(w.is_ok(), true);
    }

    #[test]
    fn test_put_get() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut db = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        db.put(b"my key", b"my value");
        assert_eq!(db.get(b"my key").unwrap().unwrap(), "my value".as_bytes().to_vec());
    }

    #[test]
    fn test_put_and_get_ok() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut d = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        d.put("a".as_bytes(), "b".as_bytes());
        let res = d.get("a".as_bytes()).unwrap();
        assert_eq!(res.is_some(), true);
    }

    #[test]
    fn test_delete() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut db = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        db.put(b"my key", b"my value");
        db.delete(b"my key");
        assert_eq!(db.get(b"my key").unwrap(), None);
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

}
