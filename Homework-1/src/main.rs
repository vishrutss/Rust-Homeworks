//! Command-line modular exponentation tool
//!
//! Vishrut Sharma 2023

// I have collaborated with Shrikrishna Bhat for this Homework

// Main function
fn main() {
    // Got the syntax to use command line arguments from:
    // https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        error()
    }
    let val1 = &args[1];
    let val2 = &args[2];
    let val3 = &args[3];

    let x = parsenum(val1);
    let y = parsenum(val2);
    let m = parsenum(val3);

    let result = modexp(x, y, m);
    print!("The result is {}", result);
}

// Function to calculate an exponential modulo
fn modexp(x: u64, y: u64, m: u64) -> u64 {
    let bigm = u128::from(m);

    // Found usage of checked_mul here: https://docs.rs/num/latest/num/trait.CheckedMul.html
    if bigm == 0 || (bigm - 1).checked_mul(2).is_none() {
        panic!("Error")
    }

    if m == 1 {
        return 0;
    }
    let mut z: u128 = 1;
    let mut bigx: u128 = u128::from(x) % bigm;
    let mut bigy: u128 = u128::from(y);
    while bigy > 0 {
        if bigy % 2 == 1 {
            z = (z * bigx) % bigm;
        }
        bigy /= 2;
        bigx = (bigx * bigx) % bigm;
    }
    u64::try_from(z).unwrap()
}

/// Print a usage error message and exit.
fn error() -> ! {
    eprintln!("modexp usage: modexp <x> <y> <m>");
    std::process::exit(1);
}

/// Parse the given string as a `u64`.
fn parsenum(s: &str) -> u64 {
    s.parse().unwrap_or_else(|_| error())
}

// Test function
#[test]
fn test_modexp() {
    // Largest prime less than 2**64.
    // https://primes.utm.edu/lists/2small/0bit.html
    let bigm = u64::max_value() - 58;
    assert_eq!(0, modexp(bigm - 2, bigm - 1, 1));
    assert_eq!(1, modexp(bigm - 2, bigm - 1, bigm));
    assert_eq!(827419628471527655, modexp(bigm - 2, (1 << 32) + 1, bigm));
    // https://practice.geeksforgeeks.org/problems/
    //    modular-exponentiation-for-large-numbers/0
    assert_eq!(4, modexp(10, 9, 6));
    assert_eq!(34, modexp(450, 768, 517));
}
