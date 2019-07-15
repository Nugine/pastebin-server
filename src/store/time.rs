use std::time::{SystemTime, UNIX_EPOCH};

pub type NanoTime = u128;
pub type SecTime = u64;

#[inline]
pub fn now_nano() -> NanoTime {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // assert: now is after UNIX_EPOCH
        .as_nanos()
}

#[inline]
pub fn sec_to_nano(sec:SecTime)->NanoTime{
    (sec as NanoTime) * 1000000
}

#[inline]
pub fn nano_to_sec(nano:NanoTime)->SecTime{
    (nano/1000000) as SecTime
}

#[cfg(test)]
#[test]
fn test_now_nano() {
    for _ in 0..3 {
        println!("now: {} ns", now_nano());
    }
}
