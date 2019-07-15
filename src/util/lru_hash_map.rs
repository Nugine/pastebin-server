use std::cmp::Eq;
use std::hash::Hash;

use linked_hash_map::LinkedHashMap;

pub trait LruValueSize {
    fn lru_value_size(&self) -> usize;
}

struct Item<V>
where
    V: LruValueSize,
{
    value: V,
    size: usize,
}

pub struct LruHashMap<K, V>
where
    K: Eq + Hash,
    V: LruValueSize,
{
    map: LinkedHashMap<K, Item<V>>,
    max_value_size: usize,
    total_value_size: usize,
}

impl<V> Item<V>
where
    V: LruValueSize,
{
    #[inline]
    fn new(value: V) -> Self {
        let size = LruValueSize::lru_value_size(&value);
        Self { value, size }
    }
}

impl<K, V> LruHashMap<K, V>
where
    K: Eq + Hash,
    V: LruValueSize,
{
    pub fn with_capacity(max_value_size: usize, capacity: usize) -> Self {
        Self {
            map: LinkedHashMap::with_capacity(capacity),
            max_value_size,
            total_value_size: 0,
        }
    }

    #[inline]
    pub fn total_value_size(&self) -> usize {
        self.total_value_size
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let item = Item::new(value);
        assert!(item.size <= self.max_value_size);
        self.pop_unitl_size(self.max_value_size - item.size);
        self.total_value_size += item.size;
        self.map.insert(key, item).map(|item| item.value)
    }

    #[inline]
    pub fn get_refresh(&mut self, key: &K) -> Option<&mut V> {
        self.map.get_refresh(key).map(|item| &mut item.value)
    }

    fn pop_unitl_size(&mut self, target_size: usize) {
        if self.total_value_size <= target_size {
            return;
        }
        if target_size == 0 {
            self.map.clear();
            self.total_value_size = 0;
            return;
        }
        while let Some((_, Item { value, size })) = self.map.pop_front() {
            self.total_value_size -= size;
            drop(value);
            if self.total_value_size <= target_size {
                break;
            }
        }
    }

    #[inline]
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|item| {
            self.total_value_size -= item.size;
            item.value
        })
    }
}

#[cfg(test)]
#[test]
fn test_lru_hash_map_order() {
    #[derive(Debug, PartialEq, Eq)]
    struct Record(i64);

    impl LruValueSize for Record {
        fn lru_value_size(&self) -> usize {
            1
        }
    }

    let n = 20_i64;
    let max_value_size = 30;
    let mut map = LruHashMap::with_capacity(max_value_size, 10);

    for i in 0..n {
        map.insert(i, Record(i));
    }

    // access 0, 2 , 4, ..., 18
    for i in (0..n).step_by(2) {
        if let Some(v) = map.get_refresh(&i) {
            v.0 += 1;
        }
    }

    for i in (n)..(n * 2) {
        map.insert(i, Record(i));
    }

    assert_eq!(map.total_value_size(), max_value_size);

    for i in 0..(n * 2) {
        let o = map.get_refresh(&i);
        if i < n {
            if i % 2 == 0 {
                assert_eq!(o, Some(&mut Record(i + 1)));
            } else {
                assert_eq!(o, None);
            }
        } else {
            assert_eq!(o, Some(&mut Record(i)));
        }
    }
}
