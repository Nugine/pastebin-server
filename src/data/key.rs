use crate::env::CRYPT_KEY;
use crate::time::NanoTime;

use short_crypt::ShortCrypt;

const VALID_BYTES: usize = 8; // take first 8 bytes of NanoTime

lazy_static! {
    static ref SC: ShortCrypt = { ShortCrypt::new(&*CRYPT_KEY) };
}

#[inline]
pub fn nano_to_key(nano: NanoTime) -> String {
    SC.encrypt_to_qr_code_alphanumeric(&nano.to_ne_bytes()[..VALID_BYTES])
}

pub fn key_to_nano(key: &str) -> Option<NanoTime> {
    SC.decrypt_qr_code_alphanumeric(key)
        .ok()
        .and_then(|v: Vec<u8>| {
            if v.len() == VALID_BYTES {
                Some(v)
            } else {
                None
            }
        })
        .map(|v: Vec<u8>| {
            let mut arr: [u8; 16] = [0; 16];
            arr[..VALID_BYTES].copy_from_slice(v.as_slice());
            u128::from_ne_bytes(arr)
        })
}

#[cfg(test)]
#[test]
fn test_key_nano() {
    use crate::time::now_nano;
    for _ in 0..3 {
        let nano = now_nano();
        let key = nano_to_key(nano);
        let nano2 = key_to_nano(&key).unwrap();
        assert_eq!(nano, nano2);
        println!("nano: {}\nkey:  {}", nano, key);
        println!("arr:  {:?}", nano.to_ne_bytes());
    }
}
