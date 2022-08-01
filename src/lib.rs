#![crate_name = "linkedhashmap"]
#![allow(dead_code)]

//! Provides an easy interface to preserve the insertion order of your `HashMap`.

/// `LinkedHashMap` data structure lives here.
pub mod ds {
    use core::fmt;
    use std::collections::HashMap;
    use std::cmp::Eq;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::marker::Copy;

    /// Stores an index for quick key lookup and the value.
    struct LinkedHashMapValue<V> {
        _index: usize,
        _value: V,
    }

    /// Stores number of keys, keys in order, and values.
    pub struct LinkedHashMap<K, V> {
        _len: usize,
        _keys: Vec<K>,
        _values: HashMap<K, LinkedHashMapValue<V>>,
    }
    
    impl<K, V> LinkedHashMap<K, V> where K: Eq + Hash + Copy, V: Copy {
        /// Creates new `LinkedHashMap`.
        pub fn new() -> Self {
            return LinkedHashMap {
                _len: 0,
                _keys: Vec::new(),
                _values: HashMap::new(),
            };
        }

        /// Gets value using key; returns `Some(v)` if exists or `None`.
        pub fn get(&self, k: K) -> Option<V> {
            let value = self._values.get(&k);
            if value.is_none() {
                return None;
            }
            
            return Some(value.unwrap()._value);
        }

        /// Sets value; upserts if exists already or adds new entry.
        pub fn set(&mut self, k: K, v: V) {
            if self._values.contains_key(&k) {
                let value: &LinkedHashMapValue<V> = self._values.get(&k).unwrap();
                self._values.insert(k, LinkedHashMapValue {
                    _index: value._index,
                    _value: v,
                });
            } else {
                self._keys.push(k);
                self._values.insert(k, LinkedHashMapValue {
                    _index: self._len,
                    _value: v,
                });
                self._len += 1;
            }
        }
        
        /// Gets value using index; returns `Some(v)` if exists or `None`.
        pub fn at(&self, i: usize) -> Option<V> {
            if i >= self._len {
                return None;
            }

            return Some(self._values.get(self._keys.get(i).unwrap()).unwrap()._value)
        }

        // Sets value at index.
        pub fn set_at(&mut self, i: usize, k: K, v: V) {
            if i >= self._len {
                return;
            }

            self._keys[i] = k;
            self._values.insert(k, LinkedHashMapValue {
                _index: i,
                _value: v,
            });
        }

        /// Removes value; returns `Some(v)` if exists or `None`.
        pub fn remove(&mut self, k: K) -> Option<V> {
            if self._values.contains_key(&k) {
                let removed: LinkedHashMapValue<V> = self._values.remove(&k).unwrap();
                self._keys.remove(removed._index);
                self._len -= 1;
                
                return Some(removed._value);
            }

            return None;
        }

        /// Clears all values.
        pub fn clear(&mut self) {
            self._keys.clear();
            self._values.clear();
            self._len = 0;
        }
        
        /// Gets length.
        pub fn len(&self) -> usize {
            return self._len;
        }

        /// Check if contains a key.
        pub fn contains_key(&self, k: K) -> bool {
            return self._values.contains_key(&k);
        }

        /// Gets all keys.
        pub fn keys(&self) -> Vec<K> {
            return self._keys.clone();
        }

        /// Gets all values.
        pub fn values(&self) -> Vec<V> {
            let mut rvs: Vec<V> = Vec::new();
            for k in self._keys.iter() {
                rvs.push(self._values.get(k).unwrap()._value);   
            }
            return rvs;
        }

        /// Iterator.
        pub fn iter(&self) -> impl Iterator<Item=(K, V)> + '_ {
            let mut rvs: Vec<(K, V)> = Vec::new();
            for k in self._keys.iter() {
                rvs.push((*k, self._values.get(k).unwrap()._value));   
            }
            return rvs.into_iter();
        }
    }

    impl<K, V> fmt::Debug for LinkedHashMap<K, V> where K: Eq + Hash + Copy + Debug, V: Copy + Debug {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut out: String = String::new();
            for (k, v) in self.iter() {
                out += format!("{:?}: {:?}", k, v).as_str();
            }

            return write!(f, "{}", out);
        }
    }
}

#[cfg(test)]
mod tests {
    mod linked_hashmap {
        mod linked_hashmap {
            use crate::ds::*;
    
            #[test]
            fn new() {
                let ins = self::LinkedHashMap::<&str, usize>::new();
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
            }

            #[test]
            fn get() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                assert!(ins.get("k") == None);
                ins.set("k", 1);
                assert!(ins.get("k") == Some(1));
            }
            
            #[test]
            fn set() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                ins.set("k", 1);
                assert!(ins.len() == 1);
                assert!(ins.keys().len() == 1);
                assert!(ins.values().len() == 1);
                assert!(ins.get("k") == Some(1));
            }

            #[test]
            fn at() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                assert!(ins.at(0) == None);
                ins.set("k", 1);
                assert!(ins.at(0) == Some(1));
                assert!(ins.at(1) == None);
            }

            #[test]
            fn set_at() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                ins.set_at(1, "a", 2);
                assert!(ins.get("a") == None);
                ins.set("k", 1);
                ins.set_at(0, "b", 3);
                assert!(ins.at(0) == Some(3));
                assert!(ins.get("b") == Some(3));
            }
            
            #[test]
            fn remove() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                assert!(ins.remove("k") == None);
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
                ins.set("k", 1);
                assert!(ins.remove("k") == Some(1));
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
            }
            
            #[test]
            fn clear() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                ins.clear();
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
                ins.set("k", 1);
                ins.clear();
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
            }
            
            #[test]
            fn len() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                assert!(ins.len() == 0);
                ins.clear();
                assert!(ins.len() == 0);
                ins.set("k", 1);
                assert!(ins.len() == 1);
                ins.clear();
                assert!(ins.len() == 0);
            }
            
            #[test]
            fn contains_key() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                assert!(ins.contains_key("k") == false);
                ins.set("k", 1);
                assert!(ins.contains_key("k") == true);
            }
            
            #[test]
            fn keys() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                let mut keys: Vec<&str> = Vec::new();
                assert!(ins.keys().eq(&keys));
                ins.set("k", 1);
                keys.push("k");
                assert!(ins.keys().eq(&keys));
            }
            
            #[test]
            fn values() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                let mut values: Vec<usize> = Vec::new();
                assert!(ins.values().eq(&values));
                ins.set("k", 1);
                values.push(1);
                assert!(ins.values().eq(&values));
            }
        }

        mod debug {
            use crate::ds::*;

            #[test]
            fn fmt() {
                let mut ins = self::LinkedHashMap::<&str, usize>::new();
                assert!("" == format!("{:?}", ins));
                ins.set("k", 1);
                println!("{:?}", ins);
                assert!("\"k\": 1" == format!("{:?}", ins));
            }
        }
    }
}
