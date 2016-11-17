use std::collections::BTreeMap;


trait CanSerialiaze {
    fn serialize(&self) -> Vec<u8>;
}

impl<'a> CanSerialiaze for BTreeMap<&'a [u8], &'a [u8]> {
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

	#[test]
    fn test_open_ok() {
        
    }
}