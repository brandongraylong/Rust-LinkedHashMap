#![crate_name = "indexedlinkedhashmap"]

//! Provides an easy interface to preserve the insertion order of your `HashMap`.

/// `IndexedLinkedHashMap` data structure lives here.
pub mod ds {
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::marker::Copy;

    pub trait Keys<K> {
        fn get(&self, i: usize) -> &K;
        fn set(&mut self, i: usize, k: K);
        fn push(&mut self, k: K);
        fn remove(&mut self, i: usize);
        fn clear(&mut self);
        fn len(&self) -> usize;
    }

    impl<K> Keys<K> for Vec<K> {
        fn get(&self, i: usize) -> &K {
            return &self[i];
        }

        fn set(&mut self, i: usize, k: K) {
            if i >= self.len() {
                return;
            }

            self[i] = k;
        }

        fn push(&mut self, k: K) {
            self.push(k);
        }

        fn remove(&mut self, i: usize) {
            self.remove(i);
        }

        fn clear(&mut self) {
            self.clear();
        }

        fn len(&self) -> usize {
            return self.len();
        }
    }

    /// Stores an index for quick key lookup and the value.
    #[derive(PartialEq)]
    pub struct IndexedLinkedHashMapValue<V> {
        pub _index: usize,
        pub _value: V,
    }

    /// Stores number of keys, keys in order, and values.
    pub struct IndexedLinkedHashMap<I, K, V> {
        _keys: I,
        _values: HashMap<K, IndexedLinkedHashMapValue<V>>,
    }

    impl<I, K, V> IndexedLinkedHashMap<I, K, V>
    where
        I: Keys<K> + Default,
        K: Eq + Hash + Copy,
        V: Copy,
    {
        /// Creates new `IndexedLinkedHashMap`.
        pub fn new() -> Self {
            return IndexedLinkedHashMap {
                _keys: I::default(),
                _values: HashMap::new(),
            };
        }

        pub fn get(&self, k: K) -> Option<&V> {
            return match self._values.get(&k) {
                Some(v) => Some(&v._value),
                None => None,
            };
        }

        /// Sets value; upserts if exists already or adds new entry.
        pub fn set(&mut self, k: K, v: V) {
            if self._values.contains_key(&k) {
                let value: &IndexedLinkedHashMapValue<V> = self._values.get(&k).unwrap();
                self._values.insert(
                    k,
                    IndexedLinkedHashMapValue {
                        _index: value._index,
                        _value: v,
                    },
                );
            } else {
                self._keys.push(k);
                self._values.insert(
                    k,
                    IndexedLinkedHashMapValue {
                        _index: self._keys.len() - 1,
                        _value: v,
                    },
                );
            }
        }

        pub fn at(&self, i: usize) -> Option<&V> {
            if i >= self._keys.len() {
                return None;
            }

            return match self._values.get(self._keys.get(i)) {
                Some(v) => Some(&v._value),
                None => None,
            };
        }

        /// Gets key using index; returns `Some(k)` if exists or `None`.
        pub fn key_at(&self, i: usize) -> Option<&K> {
            if i >= self._keys.len() {
                return None;
            }

            return Some(self._keys.get(i));
        }

        // Sets value at index.
        pub fn set_at(&mut self, i: usize, k: K, v: V) {
            if i >= self._keys.len() {
                return;
            }

            self._keys.set(i, k);
            self._values.insert(
                k,
                IndexedLinkedHashMapValue {
                    _index: i,
                    _value: v,
                },
            );
        }

        /// Removes value; returns `Some(v)` if exists or `None`.
        pub fn remove(&mut self, k: K) -> Option<IndexedLinkedHashMapValue<V>> {
            if self._values.contains_key(&k) {
                let removed = self._values.remove(&k).unwrap();
                self._keys.remove(removed._index);

                return Some(removed);
            }

            return None;
        }

        pub fn clear(&mut self) {
            self._keys.clear();
            self._values.clear();
        }

        pub fn len(&self) -> usize {
            return self._keys.len();
        }

        /// Check if contains a key.
        pub fn contains_key(&self, k: K) -> bool {
            return self._values.contains_key(&k);
        }

        /// Gets all keys.
        pub fn keys(&self) -> &I {
            return &self._keys;
        }

        /// Gets all values.
        pub fn values(&self) -> Vec<&IndexedLinkedHashMapValue<V>> {
            return self
                ._values
                .values()
                .collect::<Vec<&IndexedLinkedHashMapValue<V>>>();
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
                let ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
            }

            #[test]
            fn get() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!(ins.get(&"k") == None);
                ins.set("k", 1);
                assert!(ins.get(&"k") == Some(&1));
            }

            #[test]
            fn set() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                ins.set("k", 1);
                assert!(ins.len() == 1);
                assert!(ins.keys().len() == 1);
                assert!(ins.values().len() == 1);
                assert!(ins.get("k") == Some(&1));
            }

            #[test]
            fn at() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!(ins.at(0) == None);
                ins.set("k", 1);
                assert!(ins.at(0) == Some(&1));
                assert!(ins.at(1) == None);
            }

            #[test]
            fn key_at() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!(ins.at(0) == None);
                ins.set("k", 1);
                assert!(ins.key_at(0) == Some(&"k"));
                assert!(ins.key_at(1) == None);
            }

            #[test]
            fn set_at() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                ins.set_at(1, "a", 2);
                assert!(ins.get(&"a") == None);
                ins.set("k", 1);
                ins.set_at(0, "b", 3);
                assert!(ins.at(0) == Some(&3));
                assert!(ins.get(&"b") == Some(&3));
            }

            #[test]
            fn remove() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!(ins.remove("k") == None);
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
                ins.set("k", 1);
                assert!(ins.remove("k") == Some(IndexedLinkedHashMapValue {
                    _index: 0,
                    _value: 1,
                }));
                assert!(ins.len() == 0);
                assert!(ins.keys().len() == 0);
                assert!(ins.values().len() == 0);
            }

            #[test]
            fn clear() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
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
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
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
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!(ins.contains_key(&"k") == false);
                ins.set("k", 1);
                assert!(ins.contains_key(&"k") == true);
            }

            #[test]
            fn keys() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                let mut keys: Vec<&str> = Vec::new();
                assert!(ins.keys().eq(&keys));
                ins.set("k", 1);
                keys.push("k");
                assert!(ins.keys().eq(&keys));
            }

            #[test]
            fn values() {
                let mut ins = self::IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                let mut values: Vec<&IndexedLinkedHashMapValue<usize>> = Vec::new();
                assert!(ins.values().eq(&values));
                ins.set("k", 1);
                values.push(&IndexedLinkedHashMapValue {
                    _index: 0,
                    _value: 1,
                });
                assert!(ins.values().eq(&values));
            }
        }

        // mod debug {
        //     use crate::ds::*;

        //     #[test]
        //     fn fmt() {
        //         let mut ins = self::IndexedLinkedHashMap::<&str, usize>::new();
        //         assert!("" == format!("{:?}", ins));
        //         ins.set("k", 1);
        //         println!("{:?}", ins);
        //         assert!("\"k\": 1" == format!("{:?}", ins));
        //     }
        // }

        mod performance {
            use crate::ds::*;
            use rand::distributions::{Distribution, Standard};
            use std::{collections::HashMap, time::Instant};

            const VALIDATIONS: u128 = 10;
            const ITERATIONS: usize = 1000;

            fn get_random_number<T>() -> T
            where
                Standard: Distribution<T>,
            {
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
                    println!(
                        "Average of {} runs of {} iterations: {} {}",
                        VALIDATIONS,
                        ITERATIONS,
                        value / VALIDATIONS,
                        key
                    );
                }
                println!("===== END REPORT =====");
            }

            #[test]
            fn run() {
                let mut averages: HashMap<&str, u128> = HashMap::new();
                for _ in 0..VALIDATIONS {
                    let now = Instant::now();
                    let mut ins = self::IndexedLinkedHashMap::<Vec<usize>, usize, usize>::new();

                    for i in 0..ITERATIONS {
                        let k = get_random_number::<usize>();
                        let v = get_random_number::<usize>();
                        ins.set(k, v);
                        ins.get(k);
                        ins.at(i);
                        ins.key_at(i);
                        ins.set_at(i, k, v);
                        ins.len();
                        ins.contains_key(k);
                        ins.keys();
                        ins.values();
                        ins.remove(k);
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