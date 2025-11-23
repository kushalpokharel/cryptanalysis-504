use curv::{BigInt, arithmetic::Converter};
use std::io::Write;

// use square and multiply to do exponentiation
pub fn exponentiation(mut base:BigInt, mut exp:BigInt, modulus:&BigInt)->BigInt{
    let zero = BigInt::from(0);

    let mut result = BigInt::from(1);
    while exp!=zero{
        // println!("Bits : {exp}");
        
        if &exp & BigInt::from(1) != zero {
            result*=&base;
            result = result % modulus
        }
        exp = exp>>1;
        base = (&base * &base) % modulus;
    }
    result%modulus
}

fn test_prime_rabin(candidate_prime:BigInt)->bool{
    let mut n = BigInt::from(&candidate_prime-1);
    // get the even factor out of n-1 by repeatedly dividing by 2 until only odd part is left, i.e n-1 = 2^r*d where d is odd
    let mut r = 0;
    while !(&n%2!=BigInt::from(0)){
        n = n/2;
        r+=1;
    }

    // println!("n = 2^{}*{}", r, n );

    // we should randomly get an a and check if a^d=1(mod n) and if not, keep squaring a^d until we find -1 before 1. 
    // if both of these are not true we can say that the number n is composite with some confidence (1/4). 
    // Hence it is more of a composite testing than primality testing.
    // let's repeat this k number of times to increase our confidence. If each time we don't get a composite we can say it is a prime.
    // usually some random function would be good to get the numbers but we will use k itself.
    for k in 2..50{
        let mut value_1: BigInt = exponentiation(BigInt::from(k), n.clone(), &candidate_prime);
        let check_1:bool = value_1 == BigInt::from(1);
        if check_1 {
            continue;
        }
        let mut check_2:bool = true;
        let minus_one =  BigInt::from(-1)+&candidate_prime;
        let mut prev_value = value_1.clone();
        for _ in 0..r{
            // square the values to get 2^s 
            value_1 = (&value_1*&value_1)%&candidate_prime;
            if value_1 == BigInt::from(1){
                // this is the condition when the root of 1 is either -1 or 1 
                if prev_value != minus_one && prev_value != BigInt::from(1){
                    return false;
                }

            }
            prev_value = value_1.clone();
        }
        if value_1!=BigInt::from(1){
            return false;
        }
        if !(check_1 || check_2){
            return false;
        }
    }
    return true;
}

fn find_prime_greater_than(mut current: BigInt) -> BigInt{
    while !test_prime_rabin(current.clone()+1){
        current = current+1;
    }
    return current+1;
}

fn get_inverse(mut a:BigInt, mut b:BigInt)->(BigInt,BigInt){
    (a,b) =  if a<b {(b,a)} else {(a,b)};
    let (mut s1, mut s2, mut t1, mut t2) = (BigInt::from(1),BigInt::from(0),BigInt::from(0),BigInt::from(1));
    println!("Setup with a = {a} b = {b} s1 = {s1} s2={s2} t1={t1} t2 = {t2}");
    let mut i = 0;
    while &b != &BigInt::from(0){
        let c = b.clone();
        let q = &a/&b;
        b = &a % &b;
        a = c;
        let temp_s = &s1 - &q*&s2;
       (s1,s2) = (s2,temp_s);
       let temp_t = &t1 - &q*&t2;
       (t1,t2) = (t2,temp_t);
        // println!("Step {i} with quotient = {q} a = {a} b = {b} s1 = {s1} s2={s2} t1={t1} t2 = {t2}");
        i+=1
    }
    (s1,t1)
}

fn read_lines(filename: &str) -> Vec<BigInt> {
    std::fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)
        .map(|s| BigInt::from_str_radix(&s, 10).unwrap())  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn decrypt_merkle_hellman(){
    let mut f = std::fs::File::options().append(true).open("/Users/kushalpokharel/Documents/Cryptography/merklehellman_and_pgm/src/MH_plaintext").unwrap();
    let matrix=  vec![" *4>HR\\fo0", 
                "!+5?IS]gpy", 
                "\",6@JT^hqz",
                "#-7AKU_ir{",
                "$.8BLV`js|",
                "%/9CMWakt}",
                "&0:DNXblu~",
                "\'1;EOYcmv0",
                "(2<FPZdnw\n",
                ")3=GQ[e0x\r"];
    // given hints: 
    // M = the first prime greater than
    //          2036764117802210446778721319780021001
        
	// W = the first prime greater than
	//      127552671440279916013001

    // find M and W using Rabin's primality testing - based on square root of 1 must be 1 and -1 in prime number
    let hint_M = BigInt::from_str_radix("2036764117802210446778721319780021001", 10).unwrap();
    let hint_W = BigInt::from_str_radix("127552671440279916013001", 10).unwrap();
    let M =find_prime_greater_than(hint_M.clone());
    let W = find_prime_greater_than(hint_W.clone());

    println!("prime greater than {hint_M} with high confidence is {M}");
    println!("prime greater than {hint_W} with high confidence is {W}");

    // read public key b_i's from knapsack, get W's inverse in M and multiply with each b_i and sort those results to get the superincreasing
    // array which is our private key.

    let pubkey_array = read_lines("./cipher_knapsack/knapsack.txt");

    let (_, mut inv) = get_inverse(M.clone(), W.clone());
    inv = inv+&M;
    assert_eq!((&W*&inv)%&M, BigInt::from(1));
    let mut private_array:Vec<BigInt> = pubkey_array.into_iter().map(|knap| (knap*&inv)%&M).collect();
    let priv_array = private_array.clone();
    let mut index_array:Vec<(&BigInt, usize)> = priv_array.iter().zip(0..priv_array.len()).collect();
    index_array.sort();

    //sort the array to get the private_key array
    private_array.sort();

    //verify that this array is super increasing
    let mut sum = BigInt::from(0);
    for i in &private_array{
        println!("i {i} sum {sum}");
        assert!(*i>sum);
        sum+=i;
    }
    //read the cipher 
    let ciphers = read_lines("./cipher_knapsack/cipher.txt");
    // try to subtract the cipher with the sorted array elements.
    // if cipher is greater than the array elements, it should turn that bit on as the array is superincreasing
    // and if we don't use this, we will never be able to get the required sum
    private_array.reverse();

    for mut cipher in ciphers{
        let mut bit_string = vec![];
        let mut permuted_bit_string = vec!["0"; private_array.len()];
        cipher = (cipher*&inv)%&M;
        for element in &private_array{
            // println!("cipher {cipher} element {element}");
            if cipher>=*element{
                cipher=cipher-element.clone();
                bit_string.push("1");
            }
            else{
                bit_string.push("0");
            }
        }
        assert_eq!(cipher, BigInt::from(0));
        bit_string.reverse();

        // finally get the permutation of the x's arranged in y to permute the bits accordingly 
        // and get the final BigInt from which matrix can be read. from index_array
        print!("[");
        for (idx, (_, b)) in index_array.iter().enumerate(){
            print!("{b}, ");
            permuted_bit_string[*b]=bit_string[idx];
        }
        print!("]\n");
        // permuted_bit_string = permuted_bit_string.into_iter().collect::<Vec<&str>>();
        let permuted_bit_string:String = permuted_bit_string.into_iter().collect();
        let mut result = BigInt::from_str_radix(&permuted_bit_string, 2).unwrap().to_string();
        println!("result: {result} bit_string ");
        for i in bit_string{
            print!("{i}");
        }

        // given 7 length of plaintext which will be 14 digit with each row-column pair. 
        if result.len() < 14{
            result = "0".repeat(14 - result.len()) + &result;
        }
        let res:Vec<char> = result.chars().collect();
        let mut i = 0;
        let mut decrypted_string = String::from("");
        while i < res.len(){
            let row = res[i].to_digit(10).unwrap();
            let col = res[i+1].to_digit(10).unwrap();
            decrypted_string= decrypted_string+&(matrix[row as usize].as_bytes()[col as usize] as char).to_string();
            i+=2;
        }   
        println!("decrypted string {decrypted_string}");
        write!(&mut f, "{}", decrypted_string).unwrap();
    }




}
