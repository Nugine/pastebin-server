use super::time::NanoTime;
use crate::util::lru_hash_map::{LruHashMap, LruValueSize};

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd, Reverse};
use std::collections::BinaryHeap;
use std::hash::Hash;

#[derive(Debug, Eq)]
struct Expiration<K: Eq> {
    key: K,
    dead_time: NanoTime,
}

impl<K: Eq> PartialEq for Expiration<K> {
    fn eq(&self, other: &Self) -> bool {
        self.dead_time == other.dead_time
    }
}

impl<K: Eq> PartialOrd for Expiration<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dead_time.partial_cmp(&other.dead_time)
    }
}

impl<K: Eq> Ord for Expiration<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dead_time.cmp(&other.dead_time)
    }
}

#[cfg(test)]
#[test]
fn test_expiration_order() {
    let n = 100_usize;
    let mut v = vec![];
    for i in (0..n).rev() {
        let e = Expiration {
            key: i,
            dead_time: i as NanoTime,
        };
        v.push(e);
    }

    for i in 0..n {
        assert_eq!(
            v[i],
            Expiration {
                key: n - 1 - i,
                dead_time: (n - 1 - i) as NanoTime,
            }
        );
    }

    v.sort();

    for i in 0..n {
        assert_eq!(
            v[i],
            Expiration {
                key: i,
                dead_time: i as NanoTime,
            }
        );
    }
}

/**
 * priority queue of `K`, sorted by `dead_time` in ascending order
 */
pub struct ExpirationQueue<K>
where
    K: Eq + Hash,
{
    queue: BinaryHeap<Reverse<Expiration<K>>>,
}

impl<K> ExpirationQueue<K>
where
    K: Eq + Hash,
{
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: BinaryHeap::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn push(&mut self, key: K, dead_time: NanoTime) {
        self.queue.push(Reverse(Expiration { key, dead_time }))
    }

    pub fn clean<V: LruValueSize>(&mut self, now: NanoTime, map: &mut LruHashMap<K, V>) {
        while let Some(Reverse(exp)) = self.queue.peek() {
            if exp.dead_time > now {
                break;
            } else {
                map.remove(&exp.key);
                self.queue.pop();
            }
        }
    }

    #[inline]
    pub fn needs_clean(&self, now: NanoTime) -> bool {
        self.queue
            .peek()
            .map(|Reverse(exp)| exp.dead_time <= now)
            .unwrap_or(false)
    }
}

#[cfg(test)]
#[test]
fn test_expiration_queue_clean() {
    #[derive(Debug, PartialEq, Eq)]
    struct Record(u128);
    impl LruValueSize for Record {
        fn lru_value_size(&self) -> usize {
            std::mem::size_of::<Self>()
        }
    }

    let n = 100_u128;
    let mut map = LruHashMap::<u128, Record>::with_capacity(usize::max_value(), 20);
    let mut q = ExpirationQueue::with_capacity(20);
    for i in 0..n {
        q.push(i, i);
        map.insert(i, Record(i));
    }

    let now: NanoTime = 5;
    assert!(q.needs_clean(n));
    q.clean(now, &mut map);
    for i in 0..n {
        if i <= now {
            assert_eq!(map.get_refresh(&i), None)
        } else {
            assert_eq!(map.get_refresh(&i), Some(&mut Record(i)))
        }
    }
}
