use hex::FromHex;
use des_params::PC_1;

use crate::des_params::{E, IP, P, PC_2, S};

mod des_params;

struct DES{
    
}

impl DES{

    /// Extract consecutive 6-bit groups from a packed LSB-first buffer.
    ///
    /// - `src`: packed bytes (LSB-first inside each byte).
    /// - `total_bits`: number of valid bits in `src` (usually src.len()*8).
    /// - `pad_last`: if true, a final partial group (if any) is padded with zeros and returned;
    ///               if false, a final partial group is dropped.
    ///
    /// Returns Vec<u8> where each element is 0..63 (6 bits).
    pub fn groups_of_6_lsb(src: &[u8], total_bits: usize, pad_last: bool) -> Vec<u8> {
        let total_bits = total_bits.min(src.len() * 8);
        if total_bits == 0 { return Vec::new(); }

        let full_groups = total_bits / 6;
        let has_partial = (total_bits % 6) != 0;
        let n_groups = full_groups + if has_partial && pad_last { 1 } else { 0 };

        let mut out = Vec::with_capacity(n_groups);
        for g in 0..n_groups {
            let bit_index = g * 6;
            if bit_index + 6 <= total_bits {
                // full group: extract 6 bits
                let byte_idx = bit_index / 8;
                let shift = (bit_index % 8) as u16; // 0..7
                // read two bytes (or zero if missing)
                let lo = src.get(byte_idx).copied().unwrap_or(0) as u16;
                let hi = src.get(byte_idx + 1).copied().unwrap_or(0) as u16;
                let word = lo | (hi << 8);
                let val = ((word >> shift) & 0x3F) as u8;
                out.push(val);
            } else {
                // partial group: collect remaining bits and pad with zeros (LSB-first)
                let rem = total_bits - bit_index; // 1..5
                let byte_idx = bit_index / 8;
                let shift = (bit_index % 8) as u16;
                let lo = src.get(byte_idx).copied().unwrap_or(0) as u16;
                let hi = src.get(byte_idx + 1).copied().unwrap_or(0) as u16;
                let word = lo | (hi << 8);
                let raw = (word >> shift) & ((1u16 << rem) - 1);
                // pad into low bits of 6-bit value
                out.push(raw as u8);
            }
        }

        out
    }

    /// Pack 8 nibbles (4-bit values) into 4 bytes.
    ///
    /// Input: slice of length 8 where each element is in 0..=0x0F.
    /// Output: [u8;4] where out[i] = nibble[2*i] | (nibble[2*i+1] << 4).
    ///
    /// Returns Err string if the input length is not 8 or any nibble is out of range.
    pub fn pack_8_nibbles_to_4_bytes(nibbles: &[u8]) -> Result<[u8; 4], String> {
        if nibbles.len() != 8 {
            return Err(format!("expected 8 nibbles, got {}", nibbles.len()));
        }
        let mut out = [0u8; 4];
        for i in 0..4 {
            let lo = nibbles[2 * i];
            let hi = nibbles[2 * i + 1];
            if lo > 0x0F || hi > 0x0F {
                return Err(format!("nibble out of range at pair {}: {} and {}", i, lo, hi));
            }
            out[i] = (lo & 0x0F) | ((hi & 0x0F) << 4);
        }
        Ok(out)
    }

    pub fn encrypt(&mut self, mut round_key:Key, mut plaintext:String)->String{
        let length = plaintext.len()/2;
        let mut plain = Key{actual_keys:hex::decode(plaintext.clone()).expect("Error decoding the hex to buffer")};
        plaintext = hex::encode(plain.permute_with_mapping(IP.to_vec()).actual_keys);
        for i in 1..17{
            println!("Iteration ${i}");
            let ptext =plaintext.clone();
            let (x, y) = ptext.split_at(length);
            println!("Ptext :{:?}",x);
            let y_buffer = hex::decode(y).expect("Error decoding the hex to buffer");
            let x_key = Key::initialize_key_from_str(x);
            let rkey = round_key.get_k_i(i);

            let encrypted_text = y.to_owned()+&hex::encode(x_key.xor_with_other_buffer(&self.f(& rkey, y_buffer)).actual_keys);
            plaintext = encrypted_text
        }
        plaintext
    }
    pub fn f(&mut self, round_key:& Key, plaintext_buffer:Vec<u8>)->Key{
        // wrap plaintext_buffer in a Key so that we can permute it.
        let mut plaintext = Key::initialize_key_from_bytes(&plaintext_buffer);

        println!("Plaintext {:?} rounKey {:?}",plaintext, round_key);
        println!("Plaintext IP permutation");

        let permuted = plaintext.permute_with_mapping(E.to_vec());
        println!("Plaintext {:?} rounKey {:?}",permuted, round_key);
        let result = permuted.xor_with_other_buffer(round_key);
        println!("Result {:?}",result);
        let groups = DES::groups_of_6_lsb(&result.actual_keys, 48, false);
        let mut total_output:Vec<u8>= vec![];
        for (ind,grp) in groups.iter().enumerate(){
            let col_no = (grp&30)>>1;
            let row_no = grp&1+((grp)>>5)*2;
            println!("{:?} {:?} {:?}",grp, col_no, row_no);

            let output: u8 = S[ind][row_no as usize][col_no as usize];
            total_output.push(output);
        }   
        let final_output = DES::pack_8_nibbles_to_4_bytes(&total_output).expect("Somethihng wrong while converting nibble to bytes");
        let mut key_final_output = Key{actual_keys:final_output.to_vec()};
        println!("Final IP inverse permutation");

        let permuted_final_output = key_final_output.permute_with_mapping(P.to_vec());
        permuted_final_output
    }
}

trait Permutable {
    fn permute_with_mapping(&mut self, mapping:Vec<i32>)->Key;
    fn permute_with_reverse_mapping(&mut self, mapping:Vec<i32>)->Key;
}

#[derive(Debug)]
struct Key{
    actual_keys: Vec<u8>
}

impl Key{

    pub fn xor_with_other_buffer(&self, other: & Key)->Key{
        let mut out = vec![0u8;self.actual_keys.len()];
        println!("Length {:?} {:?}", self.actual_keys.len(), other.actual_keys.len());
        for i in 0..self.actual_keys.len(){
            out[i] = self.actual_keys[i]^other.actual_keys[i];
        }
        Key{actual_keys:out}
    }

    /// Get a bit from byte-slice treating bit index 0 as MSB of bytes[0].
    fn get_bit_msb(&self, bit_index_zero_based: usize) -> bool {
        let byte_idx = bit_index_zero_based / 8;
        let bit =  bit_index_zero_based % 8;
        ((self.actual_keys[byte_idx] >> bit) & 1) != 0
    }

    /// Set a bit in a mutable byte-slice (MSB-first).
    fn set_bit_msb(&self, output_byte: &mut [u8], bit_index_zero_based: usize, value: bool) {
        let byte_idx = bit_index_zero_based / 8;
        let bit = bit_index_zero_based % 8;
        if value {
            output_byte[byte_idx] |= 1 << bit;
        } else {
            output_byte[byte_idx] &= !(1 << bit);
        }
    }

    fn initialize_key_from_str(key:&str)->Self{

        let bytes = Vec::<u8>::from_hex(key).expect("Not convertible to key");
        return Key{
            actual_keys: bytes,
        }
    }
    fn initialize_key_from_bytes(key:&[u8])->Self{

        return Key { actual_keys: key.to_vec() }
    }

    fn split_at_middle(&self)->(Key, Key){
        let a = hex::encode(self.actual_keys.clone());
        println!("{:?}", a);
        let (msb, lsb) = a.split_at(a.len()/2);
        let l = if lsb.len()%2==1 {"0".to_string()+lsb} else {lsb.to_string()};
        let m = if lsb.len()%2==1 {"0".to_string()+msb} else {msb.to_string()};
        return (Key::initialize_key_from_str(&m), Key::initialize_key_from_str(&l));
    }

    /// Logical left shift (LSB-first) on a packed byte buffer.
    /// - `src`: input bytes (LSB-first inside each byte).
    /// - `total_bits`: how many bits in src are meaningful (<= src.len()*8).
    /// - `shift`: number of bits to left-shift (>=0).
    /// Returns a Vec<u8> of the same byte length as src (trailing partial bits masked to zero).
    pub fn shift_left_lsb(src: &[u8], total_bits: usize, shift: usize) -> Vec<u8> {
        if shift == 0 {
            return src.to_vec();
        }
        let nbytes = src.len();
        if nbytes == 0 { return Vec::new(); }

        let total_bits = total_bits.min(nbytes * 8);
        if shift >= total_bits {
            // everything shifted out -> all zeros, sized to contain total_bits bits
            return vec![0u8; nbytes];
        }

        let whole = shift / 8;
        let rem = (shift % 8) as u8; // 0..7

        let mut out = vec![0u8; nbytes];

        if rem == 0 {
            // byte-aligned shift
            for j in 0..nbytes {
                let src_idx = j as isize - whole as isize;
                out[j] = if src_idx >= 0 && (src_idx as usize) < nbytes {
                    src[src_idx as usize]
                } else {
                    0
                };
            }
        } else {
            // non-aligned: combine parts from src[src_idx] and src[src_idx - 1]
            let inv = 8 - rem;
            for j in 0..nbytes {
                let src_idx = j as isize - whole as isize;
                // low part from src[src_idx] shifted left by rem
                let low = if src_idx >= 0 && (src_idx as usize) < nbytes {
                    src[src_idx as usize].wrapping_shl(rem.into())
                } else {
                    0
                };
                // carry-in from previous (lower-index) source byte:
                let high = if (src_idx - 1) >= 0 && ((src_idx - 1) as usize) < nbytes {
                    src[(src_idx - 1) as usize].wrapping_shr(inv.into())
                } else {
                    0
                };
                out[j] = (low | high) & 0xFF;
            }
        }

        // Mask out any bits above total_bits in the final (highest) byte.
        let last_bit_count = total_bits % 8;
        if last_bit_count != 0 {
            let last_idx = (total_bits - 1) / 8;
            let mask: u8 = ((1u16 << last_bit_count) - 1) as u8;
            out[last_idx] &= mask;
            // zero higher bytes (shouldn't be necessary but safe)
            for k in (last_idx + 1)..nbytes {
                out[k] = 0;
            }
        } else {
            // if total_bits aligns exactly on a byte boundary, bytes after that (if any) should already be zero
            let boundary_byte = total_bits / 8;
            for k in boundary_byte..nbytes {
                out[k] = 0;
            }
        }

        out
    }

    fn get_k_i(&mut self, i:i32)->Key{
        let (a, b) = self.split_at_middle();
        // println!("Splitted and got {:?} {:?}", a, b);
        let mut a_shift_key:Vec<u8>;
        let mut b_shift_key:Vec<u8>;
        if i==1||i==2||i==9||i==16{
            a_shift_key = Key::shift_left_lsb(&a.actual_keys.clone(), 56, 1 );
            b_shift_key = Key::shift_left_lsb(&b.actual_keys.clone(), 56, 1 );
        }
        else{
            a_shift_key = Key::shift_left_lsb(&a.actual_keys.clone(), 56, 2);
            b_shift_key = Key::shift_left_lsb(&b.actual_keys.clone(), 56, 2 );
        }
        // println!("Shifted byte {:?} {:?}",a_shift_key , b_shift_key);
        a_shift_key.append(&mut b_shift_key);
        // println!("Appended key: {:?}", a_shift_key);

        // first update self.actual_keys to the newly shifted/combined 56-bit value
        // so that permute_with_mapping reads from the correct source bits.
        self.actual_keys = a_shift_key;
        println!("Key permutation 2");

        let permuted_key = self.permute_with_mapping(PC_2.to_vec());
        // println!("Permuted key: {:?}", permuted_key);
        
        return permuted_key;
    }
}

impl Permutable for Key{
    /// Permute bits from `src_bytes` according to `mapping` (1-based values).
    /// Returns a new Vec<u8> containing packed bits (MSB-first).
    fn permute_with_mapping(&mut self, mapping: Vec<i32>)->Key{
        let out_bits = mapping.len();
        let out_len = (out_bits + 7) / 8;
        // allocate output buffer (initialized to zero)
        let mut out: Vec<u8> = vec![0u8; out_len];

        // debug: print a few diagnostics to help reason about zero output
        for (out_pos, &m) in mapping.iter().enumerate() {
            let src_bit_idx = (m as usize) - 1; // zero-based
            let src_byte_idx = src_bit_idx / 8;
            if src_byte_idx >= self.actual_keys.len() {
                eprintln!("Mapping index {} (src bit {}) out of bounds for input of {} bytes", m, src_bit_idx, self.actual_keys.len());
                continue;
            }
            let src_byte = self.actual_keys[src_byte_idx];
            let bit_val = self.get_bit_msb(src_bit_idx);
            // Print only the first 16 mapped bits to avoid huge output
            // if out_pos < 16 {
            //     println!("mapping[{}] = {} -> src_bit {} (byte {} = {:08b}) => bit_val={}", out_pos, m, src_bit_idx, src_byte_idx, src_byte, bit_val);
            // }
            self.set_bit_msb(&mut out, out_pos, bit_val);
        }
        Key{actual_keys:out}
    }
    fn permute_with_reverse_mapping(&mut self, mapping: Vec<i32>)->Key{
        println!("Permutation output for mapping ");
        let out_bits = mapping.len();
        let out_len = (out_bits + 7) / 8;
        // allocate output buffer (initialized to zero)
        let mut out: Vec<u8> = vec![0u8; out_len];

        // debug: print a few diagnostics to help reason about zero output
        // mapping semantic (this variant): mapping[i] = destination (1-based) for the bit
        // at source index i (0-based). Enumerate yields (src_index, &dest_one_based).
        for (src_idx, &dest_one_based) in mapping.iter().enumerate() {
            let src_bit_idx = src_idx; // zero-based source bit index
            // ensure source bit exists
            if src_bit_idx >= self.actual_keys.len() * 8 {
                eprintln!("Source bit index {} out of range for input of {} bytes", src_bit_idx, self.actual_keys.len());
                continue;
            }
            let bit_val = self.get_bit_msb(src_bit_idx);

            // convert destination to zero-based and guard bounds for output
            if dest_one_based <= 0 {
                eprintln!("Invalid destination mapping value {} at src {}", dest_one_based, src_idx);
                continue;
            }
            let dest_zero = (dest_one_based as usize) - 1;
            if dest_zero >= out_bits {
                eprintln!("Destination {} (zero {}) out of range for output bits {}", dest_one_based, dest_zero, out_bits);
                continue;
            }
            self.set_bit_msb(&mut out, dest_zero, bit_val);
        }
        Key{actual_keys:out}
    }
}

fn main(){
    let key_str = "133457799BBCDFF1";
    let plain_text = "0123456789ABCDEF";
    let mut key = Key::initialize_key_from_str(&key_str);
    println!("Key permutation");
    key = key.permute_with_mapping(PC_1.to_vec());
    let mut des = DES{};
    let encrypted = des.encrypt(key, plain_text.to_owned());
    println!("Encrypted string: {:?}", encrypted);
    let mut enc_key = Key::initialize_key_from_str(&encrypted);
    let encryptedval = enc_key.permute_with_reverse_mapping(IP.to_vec());
    println!("Encrypted string: {:?}", hex::encode(encryptedval.actual_keys));

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_key() {
        let key_str = "B2A7FF25E98C35D8";
                    //                                         32  28   24   20   16   12   8    4
                    //1011 0010 1010 0111 1111 1111 0010 0101 1110 1001 1000 1100 0011 0101 1101 1000
                    // B.   2.    A.   7.  F.    F.   2.   5.   E.   9.   8.   C.   3.   5.   D.   8
        let mut key = Key::initialize_key_from_str(&key_str);
        key.permute_with_mapping(PC_1.to_vec());
        println!("{:?} {:?}",key.actual_keys, hex::encode(key.actual_keys.clone()));
    }

    #[test]
    fn test_key_permutation() {
        let key_str = "02";
        let mut key = Key::initialize_key_from_str(key_str);
        println!("{:?}",key.actual_keys);
        let mapping = [2,3,4,1];
        key.permute_with_mapping(mapping.to_vec());
        println!("{:?}",key.actual_keys);

        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(3, 3);
    }

    #[test]
    fn test_first_key(){
        let key_str = "B2A7FF25E98C35D8";
                    //                                         32  28   24   20   16   12   8    4
                    //1011 0010 1010 0111 1111 1111 0010 0101 1110 1001 1000 1100 0011 0101 1101 1000
                    // B.   2.    A.   7.  F.    F.   2.   5.   E.   9.   8.   C.   3.   5.   D.   8
        let mut key = Key::initialize_key_from_str(&key_str);
        key.permute_with_mapping(PC_1.to_vec());
        println!("{:?} {:?}",key.actual_keys, hex::encode(key.actual_keys.clone()));
        key.get_k_i(1);
    }
    #[test]
    fn test_two_keys(){
        let key_str = "B2A7FF25E98C35D8";
                    //                                         32  28   24   20   16   12   8    4
                    //1011 0010 1010 0111 1111 1111 0010 0101 1110 1001 1000 1100 0011 0101 1101 1000
                    // B.   2.    A.   7.  F.    F.   2.   5.   E.   9.   8.   C.   3.   5.   D.   8
        let mut key = Key::initialize_key_from_str(&key_str);
        key.permute_with_mapping(PC_1.to_vec());
        println!("{:?} {:?}",key.actual_keys, hex::encode(key.actual_keys.clone()));
        key.get_k_i(1);
        key.get_k_i(2);
    }

    #[test]
    fn permute_unpermute(){
        let some_string = "B2A7FF25E98C35D8";
        let mut key = Key::initialize_key_from_str(some_string);
        key = key.permute_with_mapping(IP.to_vec());
        key = key.permute_with_reverse_mapping(IP.to_vec());
        println!("{:?}", hex::encode(key.actual_keys));
    }
}