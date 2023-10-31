use std::collections::HashMap;

pub fn map_entries<K, V, Output>(map: HashMap<K, V>) -> Output
where
  Output: From<Vec<(K, V)>>
{
  let mut entries = Vec::with_capacity(map.len());
  for (key, value) in map.into_iter() {
    entries.push((key, value))
  };
  Output::from(entries)
}