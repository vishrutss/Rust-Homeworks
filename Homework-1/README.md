# Rust-Homeworks
## Name
Homework 1: Modular Exponentiation

## Submitted by
Vishrut Sharma

## Description
This code is a command-line tool for performing modular exponentiation. The main() function takes command line arguments (three values for the base, exponent, and modulo) and then calls the modexp() function to perform the modular exponentiation calculation. The modexp() function calculates the value of x raised to the power of y, modulo m.

The implementation uses a loop to calculate the value of the modular exponentiation. It first converts m to a 128-bit integer to prevent overflow. It then performs the calculation by repeatedly squaring x, taking the modulus of the result with m, and dividing y by 2 until y reaches 0. Finally, the result is converted back to a 64-bit integer.

The code also includes an error() function that prints a usage error message and exits the program, a parsenum() function that parses a string as a u64 or prints an error message, and a test_modexp() function for testing the correctness of the modular exponentiation function using various test cases.

The following pseudocode was used to create the program. Link: https://en.wikipedia.org/wiki Modular_exponentiation#Right-to-left_binary_method

    modexp(x, y, m):
        if m = 0 or (m - 1)2 would overflow
            fail
        if m = 1
            return 0
        z ← 1
        while y > 0
            if y mod 2 == 1
                z ← (z x) mod m
            y ← y div 2
            x ← x2 mod m
        return z

## Usage
To compute modexp(2,20,17) we can invoke the calculator with

    cargo run 2 20 17

where x=2, y=20, and m=17. The program should print 16 on standard output in this case.

## Testing
The following tests were used to test the program

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