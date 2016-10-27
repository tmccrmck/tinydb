pub mod tinydb;

#[cfg(test)]
mod tests {
    use tinydb;

    #[test]
    fn test_open() {
        let d = tinydb::Db::open("test.db");
        assert_eq!(d.is_ok(), true);
    }

    #[test]
    fn test_put() {
        let mut d = tinydb::Db::open("test.db").unwrap();
        let w = d.put("a".as_bytes(), "b".as_bytes()).unwrap();
        assert_eq!(w, 1);
    }
}
