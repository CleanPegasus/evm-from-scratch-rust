use std::collections::HashMap;
use primitive_types::U256;

pub struct Storage {
  storage: HashMap<U256, U256>,
  cache: Vec<U256>
}

impl Storage {

  pub fn new() -> Self {
    let mut storage = HashMap::new();
    let mut cache = Vec::new();

    Self {storage, cache}
  }

  /// Store U256 value
  pub fn store(&mut self, key: U256, value: U256) {
    self.storage.insert(key, value);
  }

  /// warm or cold load storage
  pub fn load(&mut self, key: &U256) -> (bool, U256) {
    let warm = self.cache.contains(key);
    if !warm { self.cache.push(*key) };
    return match self.storage.get_key_value(key) {
      Some((_, v)) => (warm, *v),
      None => (false, U256::zero())
    };
  }
}