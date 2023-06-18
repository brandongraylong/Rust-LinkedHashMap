# Rust IndexedLinkedHashMap

An indexable LinkedHashMap. Written in Rust.

## Notes

- Explanations of data structures for keys:
  - If you use a data structure such as a `Vec` for keys, you can index easily.
  - If you use a data structure such as a `BinaryHeap`, it doesn't make much sense to index on certain operations.
    - For example, this is how you'd call the set method: `ins.set(None, value)`.
  - If you want to use your own data structure, implement the `Keys` trait at `indexedlinkedhashmap::traits::Keys`.

## Getting Started

- [Install Rust](https://www.rust-lang.org/tools/install)

## Examples

```rust
fn main() {
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
```

```rust
fn main() {
    let mut ins = IndexedLinkedHashMap::<Vec<&str>, &str, usize>::new();
    
    ins.set("k", 1);

    assert!(ins.len() == 1);
    assert!(ins.keys().len() == 1);
    assert!(ins.values().len() == 1);
    assert!(ins.get("k") == Some(&1));
}
```

```rust
#[derive(Clone, Debug)]
struct Line2D {
    id: String,
    p1: usize,
    p2: usize,
}

fn main() {
    let mut ins = IndexedLinkedHashMap::<Vec<String>, String, Line2D>::new();
    let line = Line2D {
        id: String::from("1"),
        p1: 0,
        p2: 10,
    };

    ins.set(line.to_owned().id, line);
}
```

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut ins = IndexedLinkedHashMap::<BinaryHeap<usize>, usize, bool>::new();
    ins.set(2, false);
    ins.set(1, true);

    assert!(ins.at(Some(1)) == Some(&true));
}
```

## Generate Documentation

- Run `cargo doc`

## Run Tests

- Run `cargo test --release -- --nocapture`
