#![crate_name = "indexedlinkedhashmap"]

//! Provides an easy interface to preserve the insertion order of your `HashMap`.

use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
use traits::Keys;

/// Stores an index for quick key lookup and the value.
#[derive(PartialEq, Clone, Copy)]
pub struct IndexedLinkedHashMapValue<V> {
    pub index: Option<usize>,
    pub value: V,
}

/// Stores number of keys, keys in order, and values.
pub struct IndexedLinkedHashMap<I, K, V> {
    _keys: I,
    _values: HashMap<K, IndexedLinkedHashMapValue<V>>,
}

impl<I, K, V> IndexedLinkedHashMap<I, K, V>
where
    I: Keys<K> + Default,
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Creates new `IndexedLinkedHashMap`.
    pub fn new() -> Self {
        return IndexedLinkedHashMap {
            _keys: I::default(),
            _values: HashMap::new(),
        };
    }

    /// Gets value using key; returns `Some(v)` if exists or `None`.
    pub fn get(&self, k: K) -> Option<&V> {
        return match self._values.get(&k) {
            Some(v) => Some(&v.value),
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
                    index: value.index,
                    value: v,
                },
            );
        } else {
            self._keys.push(k.to_owned());
            self._values.insert(
                k,
                IndexedLinkedHashMapValue {
                    index: Some(self._keys.len() - 1),
                    value: v,
                },
            );
        }
    }

    /// Gets value using index; returns `Some(v)` if exists or `None`.
    pub fn at(&self, i: Option<usize>) -> Option<&V> {
        return match i {
            Some(i) => match i >= self._keys.len() {
                true => None,
                false => match self._keys.get(Some(i)) {
                    Some(k) => match self._values.get(k) {
                        Some(v) => Some(&v.value),
                        None => None,
                    },
                    None => None,
                },
            },
            None => match self._keys.get(i) {
                Some(k) => match self._values.get(k) {
                    Some(v) => Some(&v.value),
                    None => None,
                },
                None => None,
            },
        };
    }

    /// Gets key using index; returns `Some(k)` if exists or `None`.
    pub fn key_at(&self, i: Option<usize>) -> Option<&K> {
        return match i {
            Some(i) => match i >= self._keys.len() {
                true => None,
                false => self._keys.get(Some(i)),
            },
            None => self._keys.get(i),
        };
    }

    // Sets value at index.
    pub fn set_at(&mut self, i: Option<usize>, k: K, v: V) {
        return match i {
            Some(i) => match i >= self._keys.len() {
                true => (),
                false => {
                    self._keys.set(Some(i), k.to_owned());
                    self._values.insert(
                        k,
                        IndexedLinkedHashMapValue {
                            index: Some(i),
                            value: v,
                        },
                    );
                }
            },
            None => {
                self._keys.set(i, k.to_owned());
                self._values
                    .insert(k, IndexedLinkedHashMapValue { index: i, value: v });
            }
        };
    }

    /// Removes value; returns `Some(v)` if exists or `None`.
    pub fn remove(&mut self, k: K) -> Option<IndexedLinkedHashMapValue<V>> {
        if self._values.contains_key(&k) {
            let removed = self._values.remove(&k).unwrap();
            self._keys.remove(removed.index);

            return Some(removed);
        }

        return None;
    }

    /// Clears all values.
    pub fn clear(&mut self) {
        self._keys.clear();
        self._values.clear();
    }

    /// Gets length.
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

impl<I, K, V> Debug for IndexedLinkedHashMap<I, K, V>
where
    I: Keys<K> + Default,
    K: Eq + Hash + Clone + Debug,
    V: Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut out: String = String::new();
        for i in 0..self._keys.len() {
            match self._keys.get(Some(i)) {
                Some(k) => match self._values.get(k) {
                    Some(v) => out += format!("{:?}: {:?}", k, v.value).as_str(),
                    None => (),
                },
                None => (),
            };
        }

        return write!(f, "{}", out);
    }
}

pub mod traits {
    pub trait Keys<K> {
        fn get(&self, i: Option<usize>) -> Option<&K>;
        fn set(&mut self, i: Option<usize>, k: K);
        fn push(&mut self, k: K);
        fn remove(&mut self, i: Option<usize>);
        fn clear(&mut self);
        fn len(&self) -> usize;
    }
}

pub mod collections {
    use super::traits::Keys;
    use std::collections::BinaryHeap;

    impl<K> Keys<K> for Vec<K> {
        fn get(&self, i: Option<usize>) -> Option<&K> {
            return match i {
                Some(i) => match i >= self.len() {
                    true => None,
                    false => Some(&self[i]),
                },
                None => None,
            };
        }

        fn set(&mut self, i: Option<usize>, k: K) {
            match i {
                Some(i) => match i >= self.len() {
                    true => (),
                    false => {
                        self[i] = k;
                    }
                },
                None => (),
            };
        }

        fn push(&mut self, k: K) {
            self.push(k);
        }

        fn remove(&mut self, i: Option<usize>) {
            match i {
                Some(i) => match i >= self.len() {
                    true => (),
                    false => {
                        self.remove(i);
                    }
                },
                None => (),
            };
        }

        fn clear(&mut self) {
            self.clear();
        }

        fn len(&self) -> usize {
            return self.len();
        }
    }

    impl<K> Keys<K> for BinaryHeap<K>
    where
        K: Ord + Eq,
        BinaryHeap<K>: From<Vec<K>> + Clone,
    {
        fn get(&self, i: Option<usize>) -> Option<&K> {
            return match i {
                Some(i) => match i >= self.len() {
                    true => None,
                    false => {
                        for (idx, k) in self.iter().enumerate() {
                            if i == idx {
                                return Some(k);
                            }
                        }

                        return None;
                    }
                },
                None => None,
            };
        }

        fn set(&mut self, _i: Option<usize>, k: K) {
            let mut p = Vec::from(self.clone());
            p.push(k);

            self.append(&mut BinaryHeap::<K>::from(p));
        }

        fn push(&mut self, k: K) {
            self.push(k);
        }

        fn remove(&mut self, i: Option<usize>) {
            return match i {
                Some(i) => match i >= self.len() {
                    true => (),
                    false => {
                        let p: Vec<K> = Vec::from(self.clone());
                        let mut n: Vec<&K> = Vec::new();
                        for (idx, k) in p.iter().enumerate() {
                            if idx != i {
                                n.push(k);
                            }
                        }

                        self.append(&mut BinaryHeap::<K>::from(p));
                    }
                },
                None => (),
            };
        }

        fn clear(&mut self) {
            self.clear();
        }

        fn len(&self) -> usize {
            return self.len();
        }
    }
}

#[cfg(test)]
mod tests {
    mod indexedlinkedhashmap {
        use crate::*;

        #[test]
        fn new() {
            let ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            assert!(ins.len() == 0);
            assert!(ins.keys().len() == 0);
            assert!(ins.values().len() == 0);
        }

        #[test]
        fn get() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            assert!(ins.get(&"k") == None);
            ins.set("k", 1);
            assert!(ins.get(&"k") == Some(&1));
        }

        #[test]
        fn set() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            ins.set("k", 1);
            assert!(ins.len() == 1);
            assert!(ins.keys().len() == 1);
            assert!(ins.values().len() == 1);
            assert!(ins.get("k") == Some(&1));
        }

        #[test]
        fn at() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            assert!(ins.at(Some(0)) == None);
            ins.set("k", 1);
            assert!(ins.at(Some(0)) == Some(&1));
            assert!(ins.at(Some(1)) == None);
        }

        #[test]
        fn key_at() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            assert!(ins.at(Some(0)) == None);
            ins.set("k", 1);
            assert!(ins.key_at(Some(0)) == Some(&"k"));
            assert!(ins.key_at(Some(1)) == None);
        }

        #[test]
        fn set_at() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            ins.set_at(Some(1), "a", 2);
            assert!(ins.get(&"a") == None);
            ins.set("k", 1);
            ins.set_at(Some(0), "b", 3);
            assert!(ins.at(Some(0)) == Some(&3));
            assert!(ins.get(&"b") == Some(&3));
        }

        #[test]
        fn remove() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            assert!(ins.remove("k") == None);
            assert!(ins.len() == 0);
            assert!(ins.keys().len() == 0);
            assert!(ins.values().len() == 0);
            ins.set("k", 1);
            assert!(
                ins.remove("k")
                    == Some(IndexedLinkedHashMapValue {
                        index: Some(0),
                        value: 1
                    })
            );
            assert!(ins.len() == 0);
            assert!(ins.keys().len() == 0);
            assert!(ins.values().len() == 0);
        }

        #[test]
        fn clear() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
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
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
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
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            assert!(ins.contains_key(&"k") == false);
            ins.set("k", 1);
            assert!(ins.contains_key(&"k") == true);
        }

        #[test]
        fn keys() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            let mut keys: Vec<&str> = Vec::new();
            assert!(ins.keys().eq(&keys));
            ins.set("k", 1);
            keys.push("k");
            assert!(ins.keys().eq(&keys));
        }

        #[test]
        fn values() {
            let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
            let mut values: Vec<&IndexedLinkedHashMapValue<usize>> = Vec::new();
            assert!(ins.values().eq(&values));
            ins.set("k", 1);
            values.push(&IndexedLinkedHashMapValue {
                index: Some(0),
                value: 1,
            });
            assert!(ins.values().eq(&values));
        }

        mod debug {
            use crate::*;

            #[test]
            fn fmt() {
                let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
                assert!("" == format!("{:?}", ins));
                ins.set("k", 1);
                println!("{:?}", ins);
                assert!("\"k\": 1" == format!("{:?}", ins));
            }
        }

        mod performance {
            use crate::*;
            use rand::distributions::{Alphanumeric, DistString};
            use std::{collections::HashMap, time::Instant};

            const VALIDATIONS: u128 = 10;
            const ITERATIONS: usize = 1000;

            fn get_random_string() -> String {
                return Alphanumeric.sample_string(&mut rand::thread_rng(), u8::MAX.into());
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
                    let mut ins = IndexedLinkedHashMap::<Vec<String>, String, String>::new();

                    for i in 0..ITERATIONS {
                        let k: String = get_random_string();
                        let v: String = get_random_string();
                        ins.set(k.to_owned(), v.to_owned());
                        ins.get(k.to_owned());
                        ins.at(Some(i));
                        ins.key_at(Some(i));
                        ins.set_at(Some(i), k.to_owned(), v.to_owned());
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
