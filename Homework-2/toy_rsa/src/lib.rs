//! Toy RSA library
//!
//! Vishrut Sharma 2023

/// I have collaborated with Shrikrishna Bhat for this Homework

use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    let mut p: u64 = rsa_prime().into();
    let mut q: u64 = rsa_prime().into();
    while lambda(p, q) <= EXP || gcd(EXP, lambda(p, q)) != 1 {
        p = rsa_prime().into();
        q = rsa_prime().into();
    }
    (p.try_into().unwrap(), q.try_into().unwrap())
}

/// Encrypt the plaintext `msg` using the RSA public `key`
/// and return the ciphertext.
pub fn encrypt(key: u64, msg: u32) -> u64 {
    modexp(msg.into(), EXP, key)
}

/// Decrypt the cipertext `msg` using the RSA private `key`
/// and return the resulting plaintext.
pub fn decrypt(key: (u32, u32), msg: u64) -> u32 {
    let d:u64 = modinverse(EXP, lambda(key.0.into(), key.1.into()));
    let pub_key:u64=u64::from(key.0) * u64::from(key.1);
    modexp(msg, d, pub_key).try_into().unwrap()
}

/// Lambda function returning LCM of 2 numbers
pub fn lambda(p: u64, q: u64) -> u64 {
    lcm(p-1, q-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let p:u32=0xed23e6cd;
        let q:u32=0xf050a04d;
        let pub_key:u64=0xde9c5816141c8ba9;
        let msg:u32=0x12345f;
        let key_pair=(p,q);
        assert_eq!(encrypt(pub_key,msg),0x6418280e0c4d7675);
        assert_eq!(decrypt(key_pair,0x6418280e0c4d7675),0x12345f);
    }
}
