#![crate_name = "indexedlinkedhashmap"]
#![allow(dead_code)]

//! Provides an easy interface to preserve the insertion order of your `HashMap`.

/// `IndexedLinkedHashMap` data structure lives here.
pub mod ds {
    use core::fmt;
    use std::collections::HashMap;
    use std::cmp::Eq;
    use std::fmt::Debug;
    use std::hash::Hash;
    use std::clone::Clone;

    /// Stores an index for quick key lookup and the value.
    struct IndexedLinkedHashMapValue<V> {
        _index: usize,
        _value: V,
    }

    /// Stores number of keys, keys in order, and values.
    pub struct IndexedLinkedHashMap<K, V> {
        _len: usize,
        _keys: Vec<K>,
        _values: HashMap<K, IndexedLinkedHashMapValue<V>>,
    }
    
    impl<K, V> IndexedLinkedHashMap<K, V> where K: Eq + Hash + Clone, V: Clone {
        /// Creates new `IndexedLinkedHashMap`.
        pub fn new() -> Self {
            return IndexedLinkedHashMap {
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
            
            return Some(value.unwrap()._value.to_owned());
        }

        /// Sets value; upserts if exists already or adds new entry.
        pub fn set(&mut self, k: K, v: V) {
            if self._values.contains_key(&k) {
                let value: &IndexedLinkedHashMapValue<V> = self._values.get(&k).unwrap();
                self._values.insert(k, IndexedLinkedHashMapValue {
                    _index: value._index,
                    _value: v,
                });
            } else {
                self._keys.push(k.to_owned());
                self._values.insert(k, IndexedLinkedHashMapValue {
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

            return Some(self._values.get(self._keys.get(i).unwrap()).unwrap()._value.to_owned())
        }

        /// Gets key using index; returns `Some(k)` if exists or `None`.
        pub fn key_at(&self, i: usize) -> Option<K> {
            if i >= self._len {
                return None;
            }

            return Some(self._keys.get(i).unwrap().clone());
        }

        // Sets value at index.
        pub fn set_at(&mut self, i: usize, k: K, v: V) {
            if i >= self._len {
                return;
            }

            self._keys[i] = k.to_owned();
            self._values.insert(k, IndexedLinkedHashMapValue {
                _index: i,
                _value: v,
            });
        }

        /// Removes value; returns `Some(v)` if exists or `None`.
        pub fn remove(&mut self, k: K) -> Option<V> {
            if self._values.contains_key(&k) {
                let removed: IndexedLinkedHashMapValue<V> = self._values.remove(&k).unwrap();
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
                rvs.push(self._values.get(k).unwrap()._value.to_owned());   
            }
            return rvs;
        }

        /// Iterator.
        pub fn iter(&self) -> impl Iterator<Item=(K, V)> + '_ {
            let mut rvs: Vec<(K, V)> = Vec::new();
            for k in self._keys.iter() {
                rvs.push((k.clone(), self._values.get(k).unwrap()._value.to_owned()));   
            }
            return rvs.into_iter();
        }
    }

    impl<K, V> fmt::Debug for IndexedLinkedHashMap<K, V> where K: Eq + Hash + Clone + Debug, V: Clone + Debug {
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
                let ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
            }

            #[test]
            fn get() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                assert!(ins.get("k") == None);
                ins.set("k", 1);
                assert!(ins.get("k") == Some(1));
            }
            
            #[test]
            fn set() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                ins.set("k", 1);
                assert!(ins.len() == 1);
                assert!(ins.keys().len() == 1);
                assert!(ins.values().len() == 1);
                assert!(ins.get("k") == Some(1));
            }

            #[test]
            fn at() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                assert!(ins.at(0) == None);
                ins.set("k", 1);
                assert!(ins.at(0) == Some(1));
                assert!(ins.at(1) == None);
            }

            #[test]
            fn key_at() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                assert!(ins.at(0) == None);
                ins.set("k", 1);
                assert!(ins.key_at(0) == Some("k"));
                assert!(ins.key_at(1) == None);
            }

            #[test]
            fn set_at() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                ins.set_at(1, "a", 2);
                assert!(ins.get("a") == None);
                ins.set("k", 1);
                ins.set_at(0, "b", 3);
                assert!(ins.at(0) == Some(3));
                assert!(ins.get("b") == Some(3));
            }
            
            #[test]
            fn remove() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
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
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
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
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
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
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                assert!(ins.contains_key("k") == false);
                ins.set("k", 1);
                assert!(ins.contains_key("k") == true);
            }
            
            #[test]
            fn keys() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                let mut keys: Vec<&str> = Vec::new();
                assert!(ins.keys().eq(&keys));
                ins.set("k", 1);
                keys.push("k");
                assert!(ins.keys().eq(&keys));
            }
            
            #[test]
            fn values() {
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
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
                let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
                assert!("" == format!("{:?}", ins));
                ins.set("k", 1);
                println!("{:?}", ins);
                assert!("\"k\": 1" == format!("{:?}", ins));
            }
        }

        mod performance {
            use std::{time::Instant, collections::HashMap};
            use rand::distributions::{Alphanumeric, DistString, Standard, Distribution};
            use crate::ds::*;

            const VALIDATIONS: u128 = 10;
            const ITERATIONS: usize = 1000;

            fn get_random_string() -> String {
                return Alphanumeric.sample_string(&mut rand::thread_rng(), u8::MAX.into());
            }

            fn get_random_number<T>() -> T where Standard: Distribution<T> {
                return rand::random::<T>();
            }

            fn insert_or_add(averages: &mut HashMap<&str, u128>, key: &'static str, value: u128) {
                if averages.contains_key(key) {
                    averages.insert(key, averages.get(key).unwrap() + value);
                } else {
                    averages.insert(key, value);
                }
            }

            fn timestamp(now: &Instant, averages: &mut HashMap<&str, u128>) {
                insert_or_add(averages, "s", now.elapsed().as_secs().into());
                insert_or_add(averages, "ms", now.elapsed().as_millis());
                insert_or_add(averages, "μs", now.elapsed().as_micros());
                insert_or_add(averages, "ns", now.elapsed().as_nanos());
            }

            fn print_report(averages: &HashMap<&str, u128>) {
                println!("===== BEGIN REPORT =====");
                for (key, value) in averages.into_iter() {
                    println!("Average of {} runs of {} iterations: {} {}", VALIDATIONS, ITERATIONS, value / VALIDATIONS, key);
                }
                println!("===== END REPORT =====");
            }

            #[test]
            fn set() {
                let mut averages: HashMap<&str, u128> = HashMap::new();
                for _ in 0..VALIDATIONS {
                    let now = Instant::now();
                    let mut ins = IndexedLinkedHashMap::<String, u32>::new();

                    for i in 0..ITERATIONS {
                        let k: String = get_random_string();
                        let v: u32 = get_random_number::<u32>();
                        ins.set(k.to_owned(), v);
                        ins.get(k.to_owned());
                        ins.at(i);
                        ins.key_at(i);
                        ins.set_at(i, k.to_owned(), v);
                        ins.len();
                        ins.contains_key(k.to_owned());
                        ins.keys();
                        ins.values();
                        ins.remove(k.to_owned());
                        ins.set(k, v);
                    }
                    ins.clear();
                    timestamp(&now, &mut averages);
                }
                print_report(&averages);
            }
        }
    }
}
