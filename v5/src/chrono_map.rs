// a map with keys insertion-ordered

pub struct ChronoMap<K, V> {
	entries: Vec<(K, V)>
}

#[allow(unused)]
impl<K: PartialEq + Clone, V> ChronoMap<K, V> {
	pub fn new() -> Self {
		ChronoMap {
			entries: Vec::new()
		}
	}
	pub fn len(&self) -> usize {
		self.entries.len()
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
		for (k, v) in self.entries.iter() {
			if key == k {
				return Some(v);
			}
		}
		None
	}
	pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
		for (k, v) in self.entries.iter_mut() {
			if key == k {
				return Some(v);
			}
		}
		None
	}
	pub fn key_at(&self, index: usize) -> &K {
		&(self.entries[index].0)
	}
	pub fn set(&mut self, key: &K, value: V) {
		for (k, v) in self.entries.iter_mut() {
			if k == key {
				*v = value;
				return;
			}
		}
		self.entries.push((key.clone(), value));
	}
}

#[allow(unused)]
impl<K: Clone + PartialEq, V: Clone, const N: usize> From<[(K, V); N]> for ChronoMap<K, V> {
	fn from(array: [(K, V); N]) -> Self {
		if N == 0 {
			return ChronoMap::new();
		}
		ChronoMap {
			entries: array.to_vec()
		}
	}
}