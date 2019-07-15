use crate::data::record::Record;
use crate::store::store::Store;
use crate::store::time::NanoTime;

use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct State {
    pub store_lock: Arc<RwLock<Store<NanoTime, Record>>>,
}

impl State {
    pub fn new(max_value_size: usize, capacity: usize) -> Self {
        Self {
            store_lock: Arc::new(RwLock::new(Store::new(max_value_size, capacity))),
        }
    }
}
