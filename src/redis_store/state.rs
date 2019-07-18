use super::store;

use std::cell::RefCell;

pub type Store = store::RedisStore;

pub struct State {
    pub store: RefCell<Store>,
}

impl State {
    pub fn new(redis_url: &str) -> Self {
        let store = RefCell::new(Store::new(redis_url).expect("Can not connect to redis"));
        Self { store }
    }
}
