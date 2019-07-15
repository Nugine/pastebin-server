use crate::store::time::SecTime;
use crate::util::lru_hash_map::LruValueSize;

#[derive(Debug)]
pub struct Record {
    pub title: String,
    pub lang: String,
    pub content: String,
    pub saving_time: SecTime,
    pub expiration: SecTime,
}

impl LruValueSize for Record {
    fn lru_value_size(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.title.as_bytes().len()
            + self.lang.as_bytes().len()
            + self.content.as_bytes().len()
    }
}
