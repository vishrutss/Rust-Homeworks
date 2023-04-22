# Homework 2

## Toy RSA

## Submitted by
Vishrut Sharma

## Collaborations
Collaborated with Shrikrishna Bhat

## Description
This is a Rust library for performing RSA encryption and decryption using fixed values for the encryption exponent and prime number range. The library includes functions for generating RSA key pairs, encrypting and decrypting messages, and calculating the LCM of two numbers. The functions were taken from: http://github.com/pdx-cs-rust/toy-rsa-lib

The library defines a constant EXP that represents the fixed RSA encryption exponent. The genkey() function generates a pair of prime numbers in the range ```2**31..2**32``` that are suitable for RSA encryption with the fixed exponent. The encrypt() function takes a public key and a message and returns the encrypted ciphertext. The decrypt() function takes a private key and a ciphertext and returns the decrypted plaintext.

The library also includes a lambda() function that calculates the LCM of two numbers. This function is used internally by the genkey() and decrypt() functions.

The code follows the following pseudocode:

    E = 65537

    𝜆(p, q):
        return least common multiple of p - 1 and q - 1

    encrypt(key, msg):
        return msgE mod key

    decrypt(key = p ⋅ q, msg):
        d ← inverse of E mod 𝜆(p, q)
        return msgd mod (p ⋅ q)

    genkey:
        repeat 
            p, q ← rsa primes (primes in range 231 .. 232-1)
        until E < 𝜆(p, q) and gcd(E, 𝜆(p, q)) = 1
        return p, q

## Usage
Import the library crate using ```use toy_rsa::*;```
Call the required functions according to it's usage.

Available functions in the library crate:

1) genkey() -> (u32, u32)
2) encrypt(key: u64, msg: u32) -> u64
3) decrypt(key: (u32, u32), msg: u64) -> u32
4) lambda(p: u64, q: u64) -> u64

## Testing
For testing, I have implemented a command-line program that uses the functions defined in the library crate. The command-line program is located in ```examples/toyrsa.rs```. The program takes an input message from the user, encrypts the message, and displays the result. It then decrypts the ciphertext and displays the decrypted text. In the test section of the command-line program, I have implemented random testing using a random number generator that generates random messages which are then encrypted and decrypted.

I got the idea to use a random number generator from seeing a message from Lee Hoang on Zulip. I implemented this test inside a for loop so that multiple tests could be run.

## Sources
1) Most of the crucial functions that were required in the library crate were taken from: http://github.com/pdx-cs-rust/toy-rsa-lib
2) Got some help from chatGPT to figure out the syntax of rand functions in Rust.
3) Credit to Lee Hoang who suggested on Zulip the use of a random number generator for testing.