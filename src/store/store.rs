use super::time::NanoTime;

use std::cmp::Eq;
use std::collections::BTreeMap;
use std::hash::Hash;

use linked_hash_map::LinkedHashMap;

pub trait WithDeadTime {
    fn dead_time(&self) -> NanoTime;
}

pub trait LruValueSize {
    fn lru_value_size(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreItem<V>
where
    V: LruValueSize,
{
    pub value: V,
    pub access_count: u64,
    pub size: usize,
}

impl<V> StoreItem<V>
where
    V: LruValueSize,
{
    #[inline]
    fn new(value: V) -> Self {
        let size = LruValueSize::lru_value_size(&value);
        Self {
            value,
            access_count: 0,
            size,
        }
    }
}

pub struct Store<K, V>
where
    K: Copy + Eq + Hash,
    V: LruValueSize + WithDeadTime,
{
    map: LinkedHashMap<K, StoreItem<V>>,
    queue: BTreeMap<NanoTime, K>,
    total_value_size: usize,
    max_value_size: usize,
}

impl<K, V> Store<K, V>
where
    K: Copy + Eq + Hash,
    V: LruValueSize + WithDeadTime,
{
    pub fn new(max_value_size: usize, capacity: usize) -> Self {
        Self {
            map: LinkedHashMap::with_capacity(capacity),
            queue: BTreeMap::new(),
            total_value_size: 0,
            max_value_size,
        }
    }

    pub fn save(&mut self, key: K, value: V) {
        let item = StoreItem::new(value);
        assert!(item.size <= self.max_value_size);

        while self.max_value_size - self.total_value_size < item.size {
            if let Some((_, it)) = self.map.pop_front() {
                self.total_value_size -= it.size;
                self.queue.remove(&it.value.dead_time());
            } else {
                break;
            }
        }

        self.total_value_size+=item.size;
        let dead_time = item.value.dead_time();
        self.queue.insert(dead_time, key);
        self.map.insert(key, item);
    }

    pub fn access(&mut self, key: K) -> Option<&StoreItem<V>> {
        let item = self.map.get_refresh(&key)?;
        item.access_count += 1;
        Some(&(*item))
    }

    pub fn clean(&mut self, now: NanoTime) {
        let right = self.queue.split_off(&now);
        for (_, key) in &self.queue {
            self.map.remove(&key);
        }
        self.queue = right;
    }

    #[inline]
    pub fn needs_clean(&self, now: NanoTime) -> bool {
        self.queue
            .iter()
            .next()
            .map(|(&dead_time, _)| dead_time <= now)
            .unwrap_or(false)
    }

    #[inline]
    pub fn total_value_size(&self) -> usize {
        self.total_value_size
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

    impl WithDeadTime for Record {
        fn dead_time(&self) -> NanoTime {
            self.0
        }
    }

    let mut store = Store::new(10, 100);
    assert_eq!(store.access(1), None);

    for i in 0..20 {
        store.save(i, Record(i));
    }

    for i in 10..20 {
        let item = store.access(i).unwrap();
        assert_eq!(item.value, Record(i));
        assert_eq!(item.access_count, 1);
    }

    store.clean(15);
    for i in 10..20 {
        let v = store.access(i);
        if i >= 15 {
            let item = v.unwrap();
            assert_eq!(item.value, Record(i));
            assert_eq!(item.access_count, 2);
        } else {
            assert_eq!(v, None);
        }
    }
}
