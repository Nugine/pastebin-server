use crate::time::SecTime;

use redis::{Client, Commands, Connection, PipelineCommands, RedisResult};

pub struct RedisStore {
    conn: Connection,
}

const ACCESS_COUNT_FIELD: &'static str = "access_count";
const VALUE_FIELD: &'static str = "value";

impl RedisStore {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        let conn = Client::open(redis_url)?.get_connection()?;
        Ok(RedisStore { conn })
    }

    pub fn save(&mut self, key: &str, value: &str, expiration: SecTime) -> RedisResult<()> {
        redis::pipe()
            .atomic()
            .hset(key, ACCESS_COUNT_FIELD, 0)
            .hset(key, VALUE_FIELD, value)
            .expire(key, expiration as usize)
            .query(&mut self.conn)
    }

    pub fn access(&mut self, key: &str) -> RedisResult<Option<(u64, String)>> {
        let exists: bool = self.conn.exists(key)?;
        if !exists {
            Ok(None)
        } else {
            redis::pipe()
                .hincr(key, ACCESS_COUNT_FIELD, 1)
                .hget(key, VALUE_FIELD)
                .query(&mut self.conn)
                .map(|v| Some(v))
        }
    }

    pub fn try_reopen(&mut self, redis_url: &str) -> RedisResult<()> {
        let conn = Client::open(redis_url)?.get_connection()?;
        self.conn = conn;
        Ok(())
    }
}
