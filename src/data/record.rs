use crate::mem_store::{LruValueSize, WithDeadTime};
use crate::time::{NanoTime, SecTime};

#[derive(Debug)]
pub struct Record {
    pub title: String,
    pub lang: String,
    pub content: String,
    pub saving_time: SecTime,
    pub expiration: SecTime,
    pub dead_time: NanoTime,
}

impl LruValueSize for Record {
    fn lru_value_size(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.title.as_bytes().len()
            + self.lang.as_bytes().len()
            + self.content.as_bytes().len()
    }
}

impl WithDeadTime for Record {
    #[inline]
    fn dead_time(&self) -> NanoTime {
        self.dead_time
    }
}
