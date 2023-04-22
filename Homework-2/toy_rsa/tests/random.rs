//! Toy RSA random test program
//!
//! Vishrut Sharma 2023

use rand::Rng;
use toy_rsa::*;

/// Test function
#[test]
fn test_rsa() {
    // Got the idea to use a random number generator from seeing a message from Lee Hoang on Zulip
    // Got some help from ChatGPT on the usage of the random number generator
    for _ in 0..50 {
        let mut rng = rand::thread_rng();
        let msg: u32 = rng.gen();
        let key = genkey();
        assert_eq!(
            decrypt(key, encrypt(u64::from(key.0) * u64::from(key.1), msg)),
            msg
        );
    }
}
