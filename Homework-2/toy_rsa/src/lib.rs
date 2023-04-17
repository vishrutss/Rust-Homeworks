use toy_rsa_lib::*;

/// Fixed RSA encryption exponent.
pub const EXP: u64 = 65_537;

/// Generate a pair of primes in the range `2**31..2**32`
/// suitable for RSA encryption with exponent.
pub fn genkey() -> (u32, u32) {
    let mut p: u64 = rsa_prime().into();
    let mut q: u64 = rsa_prime().into();
    while lambda(p, q) <= EXP && gcd(EXP, lambda(p, q)) != 1 {
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
    let d = modinverse(EXP, lcm(key.0.into(), key.1.into()));
    modexp(msg, d, (key.0 * key.1).into()).try_into().unwrap()
}

/// Lambda function returning LCM of 2 numbers
pub fn lambda(p: u64, q: u64) -> u64 {
    lcm(p, q)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
