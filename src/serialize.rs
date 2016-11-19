use std::collections::BTreeMap;
use std::collections::HashMap;


trait CanSerialize<'a> {
    fn serialize(&self, map: &mut HashMap<&'a [u8], i32>) -> Vec<u8>;
}

impl<'a> CanSerialize<'a> for BTreeMap<&'a [u8], &'a [u8]> {
	fn serialize(&self, map: &mut HashMap<&'a [u8], i32>) -> Vec<u8> {
		let mut vals: Vec<u8> = Vec::new();
		let mut offset = 0;
		for (key, value) in self.iter() {
		    vals.extend(value.iter().cloned());
		    map.insert(key, offset);

		    offset += value.len() as i32;
		}
		return vals;
	}
}

#[cfg(test)]	
mod tests {
	use super::CanSerialize;
	use std::collections::BTreeMap;
	use std::collections::HashMap;

	#[test]
    fn test_vector_small() {
        let mut t = BTreeMap::new();
        let mut h: HashMap<&[u8], i32> = HashMap::new();
        t.insert("a".as_bytes(), "b".as_bytes());
        let result = t.serialize(&mut h);

        let v = "b".as_bytes().to_vec();
        assert_eq!(result[0], v[0]);
        assert_eq!(result.len(), v.len());
    }

    #[test]
    fn test_vector_large() {
        let mut t = BTreeMap::new();
        let mut h: HashMap<&[u8], i32> = HashMap::new();

        t.insert("key".as_bytes(), "auch als Neukölln-Nord oder Nord-Neukölln bezeichnet.".as_bytes());
        let result = t.serialize(&mut h);

        let v = "auch als Neukölln-Nord oder Nord-Neukölln bezeichnet.".as_bytes().to_vec();
        assert_eq!(result[0], v[0]);
        assert_eq!(result.len(), v.len());
    }

    #[test]
    fn test_hash() {
    	let mut t = BTreeMap::new();
        let mut h: HashMap<&[u8], i32> = HashMap::new();

        t.insert("a".as_bytes(), "b".as_bytes());
        t.serialize(&mut h);

        let test_key = "a".as_bytes();
        assert_eq!(h.get(&test_key), Some(&0));
    }

        #[test]
    fn test_hash_long() {
    	let mut t = BTreeMap::new();
        let mut h: HashMap<&[u8], i32> = HashMap::new();

        t.insert("0".as_bytes(), "offset0".as_bytes());
        t.insert("7".as_bytes(), "offset7".as_bytes());
        t.insert("14".as_bytes(), "offset14".as_bytes());
        t.serialize(&mut h);

        // Order is weird here because it inserts them in
        // the order offset0 => offset14 => offset7
        assert_eq!(h.get(&"0".as_bytes()), Some(&0));
        assert_eq!(h.get(&"14".as_bytes()), Some(&7));
        assert_eq!(h.get(&"7".as_bytes()), Some(&15));
    }
}