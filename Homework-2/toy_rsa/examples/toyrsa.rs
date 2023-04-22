//! Toy RSA command line program
//!
//! Vishrut Sharma 2023

use toy_rsa::*;

/// Main function
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        error()
    }
    let val = &args[1];
    let msg: u32 = parsenum(val);
    println!("Message: {}", msg);
    let key = genkey();
    println!("P: {}, Q: {}", key.0, key.1);
    let encrypted = encrypt(u64::from(key.0) * u64::from(key.1), msg);
    println!("Encrypted: {}", encrypted);
    let decrypted = decrypt(key, encrypted);
    println!("Decrypted: {}", decrypted);
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("toy rsa usage: toyrsa <msg>");
    std::process::exit(1);
}

/// Parse the given string as a `u32`.
fn parsenum(s: &str) -> u32 {
    s.parse().unwrap_or_else(|_| error())
}
