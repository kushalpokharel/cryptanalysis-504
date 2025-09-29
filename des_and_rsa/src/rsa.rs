use std::io::Write;

use curv::{arithmetic::Converter, BigInt};
use crate::rsa_parameters::MATRIX;

// use square and multiply to do exponentiation
fn exponentiation(mut base:BigInt, mut exp:BigInt, modulus:BigInt)->BigInt{
    let zero = BigInt::from(0);
    let mut result = BigInt::from(1);
    while exp!=zero{
        
        if &exp & BigInt::from(1) != zero {
            result*=&base;
            result = result % &modulus
        }
        exp = exp>>1;
        base = (&base * &base) % &modulus;
    }
    result%modulus
}

fn gcd(mut a:BigInt,mut b:BigInt)->BigInt{
    (a,b) =  if a<b {(b,a)} else {(a,b)};
    while &b != &BigInt::from(0){
        let c = b.clone();
        b = &a % &b;
        a = c
    }
    a
}

// use Pollard's p-1 factorization. Works when p-1 has small prime factors.
// because we need to sample B such that it is divisible by p-1.
// construct B by trying to get all the prime factors with correct power involved in p-1.
fn factorize_n(N:BigInt)->(BigInt, BigInt){

    let x =  BigInt::from(5);         // this should be a generator of Zq* with high probability


    // hypothesis: let's say that prime number decomposition for p-1 was upto 17.
    // construct B using prime numbers upto 5(hint from use B = 1500 or larger). let's take all the powers that could be
    // for each of those bases(primes). 
    let mut B = exponentiation(BigInt::from(2), BigInt::from(10), N.clone());
    B = B*exponentiation(BigInt::from(3), BigInt::from(10), N.clone());
    B = B*exponentiation(BigInt::from(5), BigInt::from(10), N.clone());

    let y = exponentiation(x, B, N.clone());
    println!("{:?}",y);

    let p = gcd(y-1, N.clone());
    println!("GCD {}", &p);

    // found out that g = 13 is equal to p. divide N by p to get q.
    // finally get the phi(n) = (p-1)(q-1) and find the inverse of b in 
    // multiplicative group to get the private exponent.

    let q = N/&p;
    return(p,q)


}

fn get_inverse_of_b_in_phi(mut a:BigInt, mut b:BigInt)->(BigInt,BigInt){
    (a,b) =  if a<b {(b,a)} else {(a,b)};
    let (mut s1, mut s2, mut t1, mut t2) = (BigInt::from(1),BigInt::from(0),BigInt::from(0),BigInt::from(1));

    while &b != &BigInt::from(0){
        let c = b.clone();
        let q = &a/&b;
        b = &a % &b;
        a = c;
        let temp_s = &s1 - &q*&s2;
       (s1,s2) = (s2,temp_s);
       let temp_t = &t1 - &q*&t2;
       (t1,t2) = (t2,temp_t);

    }
    (s1,t1)
}

fn read_lines(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn decrypt(ciphers:Vec<String>, private_exp:BigInt , modulus:BigInt)->Vec<BigInt>{
    let mut plain_numbers:Vec<BigInt>= vec![];
    for cipher in ciphers{
        let cipher_number = BigInt::from_str_radix(&cipher,10).unwrap();
        let plain_number = exponentiation(cipher_number.clone(), private_exp.clone(), modulus.clone());
        println!("ciphertext {cipher_number} plaintext {plain_number}");
        plain_numbers.push(plain_number)
    }
    plain_numbers
}

pub fn break_rsa(){
    let N:BigInt = BigInt::from_hex("68102916241556953901301068745501609390192169871097881297").unwrap();
    let b:BigInt = BigInt::from_hex("36639088738407540894550923202224101809992059348223191165").unwrap();
    let (p,q) = factorize_n(N.clone());
    let mut f = std::fs::File::options().append(true).open("/Users/kushalpokharel/Documents/Cryptography/des_and_rsa/src/RSA_plaintext").unwrap();
    let phi = (p-1)*(q-1);
    println!("Phi {:?}", &phi);

    let (_, mut private_exponent) = get_inverse_of_b_in_phi(b.clone() , phi.clone()) ;
    println!("{private_exponent}");
    if private_exponent<BigInt::from(0){
        private_exponent += &phi;
    }
    let ab = (&private_exponent * &b)% &phi;
    // This should equal to 1.
    assert_eq!(ab, BigInt::from(1));

    // Now get all the rows of the ciphertext (which is a number) one by one.
    // raise that number by private_exponent to get the m. ((m^b)^a = m)

    let ciphers = read_lines("/Users/kushalpokharel/Documents/Cryptography/des_and_rsa/ciphers-parameter-matrix-2/RSA-ciphertext.txt");
    let decrypted_plain_numbers = decrypt(ciphers, private_exponent, N);
    let mut user_input = String::new();
    for decrypted in decrypted_plain_numbers{
        // for each plain_number, two consecutive numbers give the row and column number in the given matrix
        println!("{decrypted}");

        let mut dec_str = decrypted.to_str_radix(10);
        println!("dec_str {dec_str} len {:?}", dec_str.len());
        if dec_str.len()%2==1{
            dec_str = "0".to_owned() + &dec_str;
        }
        let mut plaintext = String::from("");
        let mut j = 0;
        for _ in 0..dec_str.len()/2{
            let rn = dec_str.bytes().nth(j).unwrap() - 48; //(ascii-48("0"))
            let cn = dec_str.bytes().nth(j+1).unwrap() - 48;
            let string_from_matrix = MATRIX[(rn) as usize];
            println!("Indices {} {}", rn, cn);
            let character_from_matrix = string_from_matrix.chars().nth((cn) as usize).unwrap();
            plaintext = plaintext.to_string() + &character_from_matrix.to_string();
            j+=2;
            std::io::stdin().read_line(&mut user_input);
            println!("Character {}", character_from_matrix);

        }
        writeln!(&mut f, "{plaintext}").unwrap();

    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_exponentiation(){
        let y = exponentiation(BigInt::from(5), BigInt::from(2), BigInt::from(50));
        assert_eq!(y, BigInt::from(25));
    }

    #[test]
    fn test_exponentiation_bigger(){
        let y = exponentiation(BigInt::from(5), BigInt::from(4), BigInt::from(50));
        assert_eq!(y, BigInt::from(25));
    }
    #[test]
    fn test_gcd(){
        let y = gcd(BigInt::from(100), BigInt::from(76));
        assert_eq!(y, BigInt::from(4));
    }
    #[test]
    fn test_extended_eucledian(){
        let (private_exponent, _) = get_inverse_of_b_in_phi(BigInt::from(240) , BigInt::from(46));
        println!("{private_exponent}");
    }

    // #[test]
    // fn test_matrix(){
    //     let str = MATRIX[]
    //     println!("{private_exponent}");
    // }
    
}
