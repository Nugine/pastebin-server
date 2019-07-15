use crate::data::record::Record;
use crate::store::store;
use crate::store::time::NanoTime;

use std::sync::{Arc, RwLock};

pub type Store = store::Store<NanoTime, Record>;
pub type StoreLock = Arc<RwLock<Store>>;

#[derive(Clone)]
pub struct State {
    pub store_lock: StoreLock,
}

impl State {
    pub fn new(max_value_size: usize, capacity: usize) -> Self {
        Self {
            store_lock: Arc::new(RwLock::new(Store::new(max_value_size, capacity))),
        }
    }
}
