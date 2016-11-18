use std::collections::BTreeMap;


trait CanSerialize {
    fn serialize(&self) -> Vec<u8>;
}

impl<'a> CanSerialize for BTreeMap<&'a [u8], &'a [u8]> {
	fn serialize(&self) -> Vec<u8> {
		let mut v: Vec<u8> = Vec::new();
		for (key, value) in self.iter() {
		    v.extend(key.iter().cloned());
		    v.extend(value.iter().cloned());
		}
		return v;

	}
}

#[cfg(test)]
mod tests {
	use super::CanSerialize;
	use std::collections::BTreeMap;

	#[test]
    fn test1_ok() {
        let mut t = BTreeMap::new();
        t.insert("a".as_bytes(), "b".as_bytes());
        assert_eq!(t.serialize()[0], "a".as_bytes().to_vec()[0]);
        assert_eq!(t.serialize()[1], "b".as_bytes().to_vec()[0]);

        let v = "ab".as_bytes().to_vec();
        assert_eq!(t.serialize()[0], v[0]);
    }
}