use std::io::Write;


use curv::{arithmetic::Converter, BigInt};
use crate::rsa_parameters::MATRIX;

// use square and multiply to do exponentiation
pub fn exponentiation(mut base:BigInt, mut exp:BigInt, modulus:BigInt)->BigInt{
    let zero = BigInt::from(0);

    let mut result = BigInt::from(1);
    while exp!=zero{
        // println!("Bits : {exp}");
        
        if &exp & BigInt::from(1) != zero {
            result*=&base;
            result = result % &modulus
        }
        exp = exp>>1;
        base = (&base * &base) % &modulus;
    }
    result%modulus
}
// use square and multiply to do exponentiation
// (removed) exponentiation_without_modulus was unused; use exponentiation where modulus is required.

pub fn gcd(mut a:BigInt,mut b:BigInt)->BigInt{
    (a,b) =  if a<b {(b,a)} else {(a,b)};
    while &b != &BigInt::from(0){
        let c = b.clone();
        b = &a % &b;
        a = c
    }
    a
}

fn get_all_primes_upto_n(n:u64)->Vec<u64>{
    if n < 2 { return Vec::new(); }
    // create boolean sieve of size n+1 so indices match numbers
    let size = (n as usize) + 1;
    let mut is_prime = vec![true; size];
    is_prime[0] = false;
    if size > 1 { is_prime[1] = false; }

    let limit = (n as f64).sqrt() as usize;
    for i in 2..=limit {
        if is_prime[i] {
            // start marking at i*i to avoid redundant work
            let start = i * i;
            for j in (start..=n as usize).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = Vec::new();
    for i in 2..=n as usize {
        if is_prime[i] { primes.push(i as u64); }
    }
    primes
}

// use Pollard's p-1 factorization. Works when p-1 has small prime factors.
// because we need to sample B such that it is divisible by p-1.
// construct B by trying to get all the prime factors with correct power involved in p-1.
// Pollard's p-1 factorization (non-interactive). Builds a smoothness
// bound M by taking prime powers <= bound and computes a^M mod N.
// If gcd(a^M - 1, N) gives a nontrivial factor we return it. We try a few
// small bases `a` if needed.
fn factorize_p_minus_1(n: BigInt, bound: BigInt) -> Option<(BigInt, BigInt)> {
    // small prime list sufficient for moderate bounds. Extend if needed.
    let small_primes = get_all_primes_upto_n(5000);
    // Build exponent M as product of q^{e} where q^{e} <= bound
    let mut m = BigInt::from(1);
    for &q in small_primes.iter() {
        // find largest power q^e <= bound
        let mut pe: BigInt = BigInt::from(q);
        while pe <= bound 
        { 
            pe = pe*BigInt::from(q); 
        }
        m = &m * BigInt::from(pe);
    }

    println!("M {}", m);
    for base_u in 2u64..5000u64 {
        let a = BigInt::from(base_u) % &n;
        if a == BigInt::from(0) { continue; }
        let y = exponentiation(a.clone(), m.clone(), n.clone());
        let d = gcd(y - BigInt::from(1), n.clone());
        if d > BigInt::from(1) && d < n {
            let p = d.clone();
            let qv = n / &p;
            return Some((p, qv));
        }
    }
    None
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
    let n:BigInt = BigInt::from_str_radix("68102916241556953901301068745501609390192169871097881297",10).unwrap();
    let b:BigInt = BigInt::from_str_radix("36639088738407540894550923202224101809992059348223191165",10).unwrap();
    let (p,q) = match factorize_p_minus_1(n.clone(), n.clone()) {
        Some((p,q)) => (p,q),
        None => panic!("factorization failed with p-1 method; try increasing bound"),
    };
    let mut input=String::new();
    println!("p and q {p} and {q}");
    std::io::stdin().read_line(&mut input);
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
    let decrypted_plain_numbers = decrypt(ciphers, private_exponent, n.clone());
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
            // interactive pause removed for automated runs
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
