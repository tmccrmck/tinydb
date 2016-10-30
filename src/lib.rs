pub mod tinydb;

#[cfg(test)]
mod tests {
    use tinydb;

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
    fn test_put_and_get(){
        let mut d = tinydb::Db::open("test1.db").unwrap();
        let w = d.put("a".as_bytes(), "b".as_bytes());
        let res = d.get("a".as_bytes()).unwrap();
        assert_eq!(res.is_some(), true);
    }
}
