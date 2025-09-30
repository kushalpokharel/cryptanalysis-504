use curv::BigInt;

// Randomized algorithm to factor n based on square root of 1(non-trivial) 
// Used when private exponent is exposed - implication : if you expose private exponent, N could be factorized
// using a differnt private key isn't the enough then, you need to change N.
use crate::RSA::{exponentiation, gcd};


pub fn rsa_factor(n:BigInt, a:BigInt, b:BigInt)-> BigInt{
    let mut phi = a*b-1;
    println!("here {:?}", &phi>>1);
    //divide by 2 until we can to get an odd (r)
    while (&phi & BigInt::from(1)) ==BigInt::from(0){
        phi = &phi>>1;
    }
    //choose w at random, for now set some value to w.
    let w = BigInt::from(9983);
    println!("Random choice of w is {w}");
    println!("Value of s (odd) {phi}");

    let x = gcd(w.clone(), n.clone());
    if x<n && x>BigInt::from(1){
        println!("Unluckyy. use different w");
        x
    }
    else{
        let mut v = exponentiation(w.clone(), phi.clone(), n.clone());
        let mut v0 = v.clone();
        if v==BigInt::from(1){
            println!("Unlucky. use different w");
            return BigInt::from(-1);
        }
        while v!=BigInt::from(1){
            v0 = v.clone();
            println!("Value of v {v0}");

            // repeatedly power w by 2 until we reach s in which case v will be 2.
            v = exponentiation(v.clone(), BigInt::from(2), n.clone());

        }
        if &v0%&n==BigInt::from(-1){
            println!("Unable to factorize n as we didn't get the non-trivial factor but -1");
            return BigInt::from(-1)
        }
        else{
            println!("We are able to factorize n successfully using random w = {w}");
            return gcd(v0+1, n);
        }
    }
}