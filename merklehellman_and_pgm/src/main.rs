use crate::merklehellman::decrypt_merkle_hellman;
use crate::pgm::decrypt_pgm;
mod pgm;
mod merklehellman;
fn main(){
    // decrypt_merkle_hellman();  
    decrypt_pgm();
}