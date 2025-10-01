use curv::BigInt;

use crate::des as DES;
use crate::rsa as RSA;
use crate::rsa_decryption_exponent as RSA_exponent;


mod des;
mod rsa;
mod rsa_decryption_exponent;
pub mod des_params;
pub mod rsa_parameters;


fn main(){
    rsa_decryption_exponent();
}

fn rsa_decryption_exponent(){
    let n = BigInt::from(36581);
    let result = RSA_exponent::rsa_factor(n.clone(), BigInt::from(14039), BigInt::from(4679));
    if result == BigInt::from(-1) || result == BigInt::from(1) || result==BigInt::from(n.clone()){
        println!("Trivial factors of n are: {result} {:?}", {n/&result});
    }
    else{
        println!("Two factors of n are: {result} {:?}", {n/&result});

    }

}

//test