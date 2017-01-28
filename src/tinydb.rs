use std::io;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::collections::HashMap;

// use this https://github.com/carllerche/bytes

// https://ayende.com/blog/162754/reviewing-lightning-memory-mapped-database-library-partial
//https://ayende.com/blog/161410/reviewing-leveldb-part-i-what-is-this-all-about


pub struct Db<'a> {
    dir: PathBuf,
    btree: BTreeMap<Vec<u8>, Vec<u8>>,
    offsets: HashMap<&'a [u8], i32>,
    total_bytes: i32,
}

impl<'a> Db<'a> {
    pub fn open(dir: PathBuf) -> Result<Self, io::Error> {
        //let file_ = try!(File::create(path));
        
        Ok(Db {
            dir: dir,
            btree: BTreeMap::new(),
            offsets: HashMap::new(),
            total_bytes: 0,
        })
    }

    pub fn put(&mut self, key: &'a [u8], value: &'a [u8]) -> Result<usize, io::Error> {
        self.btree.insert(key.to_vec(), value.to_vec());
        self.total_bytes += key.len() as i32 + value.len() as i32;

        if self.total_bytes > 100 {

        }
        Ok(1) // placeholder
    }

    pub fn get(&mut self, key: &[u8]) -> Result<Option<Vec<u8>>, io::Error> {
        let x = self.btree.get(key);
        match x {
            Some(val) => Ok(Some(val.to_vec())),
            None => Ok(None),
        }
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.btree.remove(key);
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;

    use tinydb;
    use std::collections::BTreeMap;
    use std::fs;

    #[test]
    fn test_put_get() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut db = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        db.put(b"my key", b"my value").unwrap();
        assert_eq!(db.get(b"my key").unwrap().unwrap(), "my value".as_bytes().to_vec());
    }

    #[test]
    fn test_put_and_get_ok() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut d = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        d.put(b"a", b"b");
        let res = d.get(b"a").unwrap();
        assert_eq!(res.is_some(), true);
    }

    #[test]
    fn test_delete() {
        let tmp_dir = tempdir::TempDir::new("example").expect("create temp dir");

        let mut db = tinydb::Db::open(tmp_dir.into_path()).unwrap();
        db.put(b"my key", b"my value").unwrap();
        db.delete(b"my key");
        assert_eq!(db.get(b"my key").unwrap(), None);
    }

    // //This is old and not needed
    // // Keep in case needing to move back to serde
    // #[test]
    // fn test_serd_serialize() {
    //     let mut btree: BTreeMap<Vec<u8>, Vec<u8>> = BTreeMap::new();
    //     btree.insert(b"a".to_vec(), b"b".to_vec());

    //     let limit = bincode::SizeLimit::Bounded(1000000);
    //     let s: Vec<u8> = bincode::serde::serialize(&btree, limit).unwrap();
    //     let new_map: BTreeMap<Vec<u8>, Vec<u8>> = bincode::serde::deserialize(&s).unwrap();

    //     let key = b"a".to_vec();
    //     let val = b"b".to_vec();
    //     assert_eq!(new_map.get(&key), Some(&val));
    // }

}
