use std::{fs, io::Write};

use hex::FromHex;
mod des_params;
use des_params::{E, IP, P, PC_1, PC_2, S};

// Minimal, canonical MSB-first DES implementation focused on correctness
// for the textbook test vector (key: 133457799BBCDFF1, pt: 0123456789ABCDEF).

fn get_bit(slice: &[u8], bit_index_zero_based: usize) -> bool {
    let byte = bit_index_zero_based / 8;
    let bit = bit_index_zero_based % 8; // 0 == MSB
    if byte >= slice.len() { return false; }
    let shift = 7 - (bit as u8);
    ((slice[byte] >> shift) & 1) != 0
}
fn set_bit(slice: &mut [u8], bit_index_zero_based: usize, value: bool) {
    let byte = bit_index_zero_based / 8;
    let bit = bit_index_zero_based % 8;
    let shift = 7 - (bit as u8);
    if value { slice[byte] |= 1 << shift; } else { slice[byte] &= !(1 << shift); }
}

fn permute(src: &[u8], mapping: &[i32]) -> Vec<u8> {
    let out_bits = mapping.len();
    let out_len = (out_bits + 7) / 8;
    let mut out = vec![0u8; out_len];
    for (i, &m) in mapping.iter().enumerate() {
        if m <= 0 { continue; }
        let src_zero = (m as usize) - 1;
        let bit = get_bit(src, src_zero);
        set_bit(&mut out, i, bit);
    }
    out
}

fn rotate_left_msb(src: &[u8], total_bits: usize, shift: usize) -> Vec<u8> {
    let mut out = vec![0u8; (total_bits + 7) / 8];
    let rot = shift % total_bits;
    for i in 0..total_bits {
        let src_pos = (i + rot) % total_bits;
        if get_bit(src, src_pos) { set_bit(&mut out, i, true); }
    }
    out
}

fn build_round_keys(key: &[u8;8]) -> [[u8;6];16] {
    let pc1 = permute(key, &PC_1);
    let mut c = vec![0u8;4];
    let mut d = vec![0u8;4];
    for i in 0..28 { if get_bit(&pc1, i) { set_bit(&mut c, i, true); } }
    for i in 0..28 { if get_bit(&pc1, 28 + i) { set_bit(&mut d, i, true); } }
    let shifts = [1usize,1,2,2,2,2,2,2,1,2,2,2,2,2,2,1];
    let mut keys = [[0u8;6];16];
    for (round, &s) in shifts.iter().enumerate() {
        c = rotate_left_msb(&c, 28, s);
        d = rotate_left_msb(&d, 28, s);
        let mut cd = vec![0u8;7];
        for i in 0..28 { if get_bit(&c, i) { set_bit(&mut cd, i, true); } }
        for i in 0..28 { if get_bit(&d, i) { set_bit(&mut cd, 28 + i, true); } }
        let k = permute(&cd, &PC_2);
        let mut k6 = [0u8;6]; k6.copy_from_slice(&k[0..6]);
        keys[round] = k6;
    }
    keys
}

fn f_msb(right32: &[u8;4], round_key48: &[u8;6]) -> [u8;4] {
    let expanded = permute(right32, &E);
    let mut x = [0u8;6];
    for i in 0..6 { x[i] = expanded[i] ^ round_key48[i]; }
    let mut out32 = vec![0u8;4];
    for sidx in 0..8 {
        let mut v = 0u8;
        for b in 0..6 {
            let global = sidx*6 + b;
            if get_bit(&x, global) { v = (v << 1) | 1; } else { v = v << 1; }
        }
        let col = ((v >> 1) & 0x0F) as usize;
        let row = ((((v >> 5) & 0x01) << 1) | (v & 0x01)) as usize;
        let s_out = S[sidx][row][col];
        for k in 0..4 {
            let bit = ((s_out >> (3 - k)) & 1) != 0;
            set_bit(&mut out32, sidx*4 + k, bit);
        }
    }
    let p = permute(&out32, &P);
    let mut res = [0u8;4]; res.copy_from_slice(&p[0..4]); res
}

fn encrypt_block_msb(block: &[u8;8], round_keys: &[[u8;6];16]) -> [u8;8] {

    let ip = permute(block, &IP);
    let mut l = [0u8;4]; let mut r = [0u8;4];
    l.copy_from_slice(&ip[0..4]); r.copy_from_slice(&ip[4..8]);
    for i in 0..16 {
        let fk = f_msb(&r, &round_keys[i]);
        let mut new_r = [0u8;4];
        for j in 0..4 { new_r[j] = l[j] ^ fk[j]; }
        l = r; r = new_r;
    }
    let mut pre = vec![0u8;8]; pre[0..4].copy_from_slice(&r); pre[4..8].copy_from_slice(&l);
    let mut inv_ip = vec![0i32; IP.len()];
    for (out_pos, &src_one) in IP.iter().enumerate() { if src_one>0 { inv_ip[(src_one as usize)-1] = (out_pos+1) as i32; } }
    let final_block = permute(&pre, &inv_ip);
    let mut out = [0u8;8]; out.copy_from_slice(&final_block[0..8]); out
}

// Decrypt a single 64-bit block using the same MSB-first Feistel routine but
// with the round keys applied in reverse order.
fn decrypt_block_msb(block: &[u8;8], round_keys: &[[u8;6];16]) -> [u8;8] {
    let mut rev = [[0u8;6];16];
    for i in 0..16 { rev[i] = round_keys[15 - i]; }
    encrypt_block_msb(block, &rev)
}

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let key = <Vec<u8>>::from_hex("8B2A7FF25E98C35D").unwrap();
    let pt = <Vec<u8>>::from_hex("0123456789ABCDEF").unwrap();
    let mut key8 = [0u8;8]; key8.copy_from_slice(&key[0..8]);
    let round_keys = build_round_keys(&key8);
    let mut pt8 = [0u8;8]; pt8.copy_from_slice(&pt[0..8]);
    let ct = encrypt_block_msb(&pt8, &round_keys);
    

    let recovered = decrypt_block_msb(&ct, &round_keys);
    println!("recovered plaintext: {}", hex::encode(recovered));

    println!("ciphertext: {}", hex::encode(ct));
    let mut f = std::fs::File::options().append(true).open("/Users/kushalpokharel/Documents/Cryptography/des_and_rsa/src/DES_plaintext").unwrap();

    let lines = read_lines("/Users/kushalpokharel/Documents/Cryptography/des_and_rsa/ciphers-parameter-matrix-2/DES-ciphertext.txt");
    for line in lines{
        println!("Line {line}");
        let line_buffer = <Vec<u8>>::from_hex(line).unwrap();
        let mut line_buffer8:[u8;8] = [0u8;8]; line_buffer8.copy_from_slice(&line_buffer[0..8]);
        let decrypted_text = decrypt_block_msb(&line_buffer8, &round_keys);
        let plaintext = String::from_utf8(decrypted_text.to_vec()).unwrap();
        println!("plaintext: {plaintext}" );
        writeln!(&mut f, "{plaintext}").unwrap();

    }
}