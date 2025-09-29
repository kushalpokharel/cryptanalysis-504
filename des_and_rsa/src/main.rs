use crate::des as DES;
use crate::rsa as RSA;


mod des;
mod rsa;
pub mod des_params;
pub mod rsa_parameters;


fn main(){
    RSA::break_rsa();
}

//test