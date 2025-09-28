use des::Des;
use des::cipher::{BlockEncrypt, KeyInit};
use hex::FromHex;

fn main() {
    let key = Vec::<u8>::from_hex("133457799BBCDFF1").unwrap();
    let pt = Vec::<u8>::from_hex("0123456789ABCDEF").unwrap();
    // Des block cipher from RustCrypto
    let cipher = Des::new_from_slice(&key).expect("key length");
    let mut block_arr: [u8;8] = pt.as_slice().try_into().expect("block size");
    // create a GenericArray by cloning from slice and encrypt
    let ga_ref = des::cipher::generic_array::GenericArray::from_slice(&block_arr);
    let mut ga = ga_ref.to_owned();
    cipher.encrypt_block(&mut ga);
    // copy back
    for i in 0..8 { block_arr[i] = ga[i]; }
    println!("reference ciphertext: {}", hex::encode(block_arr));
}
