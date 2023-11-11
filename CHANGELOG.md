# IndexedLinkedHashMap

## 3.0.0

- Updated trait name from `Keys` to `Ordered`.
- Updated implementations for `Vec` and `BinaryHeap` to live under `collections::ordering` instead of `collections`.
- Added feature guards for `Vec` and `BinaryHeap` as their inclusions should be explicit.
- Removed benchmark tests as current scope should pertain to unit tests only.
