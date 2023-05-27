// a map with keys insertion-ordered

pub struct ChronoMap<K, V> {
	entries: Vec<(K, V)>,
	length: usize
}

impl<K: PartialEq, V> ChronoMap<K, V> {
	pub fn new() -> Self {
		ChronoMap {
			entries: Vec::new(),
			length: 0
		}
	}
	pub fn len(&self) -> usize {
		self.length
	}
	pub fn has_key(&self, key: &K) -> bool {
		for (k, _) in self.entries.iter() {
			if k == key {
				return true;
			}
		}
		false
	}
	pub fn get(&self, key: &K) -> Option<&V> {
		if !self.has_key(key) {
			return None;
		}
		for (k, v) in self.entries.iter() {
			if key == k {
				return Some(v);
			}
		}
		None
	}
	pub fn key_at(&self, index: usize) -> &K {
		&(self.entries[index].0)
	}
	pub fn set(&mut self, key: K, value: V) {
		if self.has_key(&key) {

		}
		self.entries.push((key, value));
	}
}

impl<K: Clone + PartialEq, V: Clone, const N: usize> From<[(K, V); N]> for ChronoMap<K, V> {
	fn from(array: [(K, V); N]) -> Self {
		if N == 0 {
			return ChronoMap::new();
		}
		ChronoMap {
			entries: array.to_vec(),
			length: array.len()
		}
	}
}