use super::expiration::ExpirationQueue;
use super::time::NanoTime;
use crate::util::lru_hash_map::{LruHashMap, LruValueSize};

use std::cmp::Eq;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct Item<V: LruValueSize> {
    pub value: V,
    pub access_count: u64,
}

impl<V: LruValueSize> Item<V> {
    #[inline]
    fn new(value: V) -> Self {
        Self {
            value,
            access_count: 0,
        }
    }
}

impl<V: LruValueSize> LruValueSize for Item<V> {
    #[inline]
    fn lru_value_size(&self) -> usize {
        self.value.lru_value_size()
    }
}

pub struct Store<K, V>
where
    K: Copy + Eq + Hash,
    V: LruValueSize,
{
    map: LruHashMap<K, Item<V>>,
    queue: ExpirationQueue<K>,
}

impl<K, V> Store<K, V>
where
    K: Copy + Eq + Hash,
    V: LruValueSize,
{
    #[inline]
    pub fn new(max_value_size: usize, capacity: usize) -> Self {
        Self {
            map: LruHashMap::with_capacity(max_value_size, capacity),
            queue: ExpirationQueue::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn save(&mut self, key: K, value: V, dead_time: NanoTime) -> Option<V> {
        self.queue.push(key, dead_time);
        let item = Item::new(value);
        self.map.insert(key, item).map(|item| item.value)
    }

    #[inline]
    pub fn access(&mut self, key: K) -> Option<&Item<V>> {
        let item = self.map.get_refresh(&key)?;
        item.access_count += 1;
        Some(&(*item))
    }

    #[inline]
    pub fn clean(&mut self, now: NanoTime) {
        self.queue.clean(now, &mut self.map)
    }

    #[inline]
    pub fn needs_clean(&self, now: NanoTime) -> bool {
        self.queue.needs_clean(now)
    }

    #[inline]
    pub fn total_value_size(&self) -> usize {
        self.map.total_value_size()
    }
}

#[cfg(test)]
#[test]
fn test_store() {
    #[derive(Debug, PartialEq, Eq)]
    struct Record(u128);
    impl LruValueSize for Record {
        fn lru_value_size(&self) -> usize {
            1
        }
    }

    let mut store = Store::new(10, 100);
    assert_eq!(store.access(1), None);

    for i in 0..20 {
        store.save(i, Record(i), i);
    }

    for i in 0..10 {
        let item = store.access(i + 10).unwrap();
        assert_eq!(item.value, Record(i + 10));
        assert_eq!(item.access_count, 1);
    }

    store.clean(15);
    for i in 0..10 {
        let v = store.access(i + 10);
        if i + 10 > 15 {
            let item = v.unwrap();
            assert_eq!(item.value, Record(i + 10));
            assert_eq!(item.access_count, 2);
        } else {
            assert_eq!(v, None);
        }
    }
}
