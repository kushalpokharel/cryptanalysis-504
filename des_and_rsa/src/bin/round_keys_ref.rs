use hex::FromHex;

// PC-1 and PC-2 tables copied here so this binary can run standalone for reference
const PC_1: [i32;56] = [
    57,49,41,33,25,17,9,
    1,58,50,42,34,26,18,
    10,2,59,51,43,35,27,
    19,11,3,60,52,44,36,
    63,55,47,39,31,23,15,
    7,62,54,46,38,30,22,
    14,6,61,53,45,37,29,
    21,13,5,28,20,12,4
];

const PC_2: [i32;48] = [
    14,17,11,24,1,5,
    3,28,15,6,21,10,
    23,19,12,4,26,8,
    16,7,27,20,13,2,
    41,52,31,37,47,55,
    30,40,51,45,33,48,
    44,49,39,56,34,53,
    46,42,50,36,29,32
];

fn get_bit_msb(slice: &[u8], bit_index: usize) -> bool {
    let byte_idx = bit_index / 8;
    if byte_idx >= slice.len() { return false; }
    let bit_in_byte = (bit_index % 8) as u8; // 0 == MSB
    let shift = 7u8 - bit_in_byte;
    ((slice[byte_idx] >> shift) & 1) != 0
}

fn set_bit_msb(slice: &mut [u8], bit_index: usize, value: bool) {
    let byte_idx = bit_index / 8;
    if byte_idx >= slice.len() { return; }
    let bit_in_byte = (bit_index % 8) as u8;
    let shift = 7u8 - bit_in_byte;
    if value {
        slice[byte_idx] |= 1 << shift;
    } else {
        slice[byte_idx] &= !(1 << shift);
    }
}

fn permute_msb(input: &[u8], mapping: &[i32]) -> Vec<u8> {
    let out_bits = mapping.len();
    let out_len = (out_bits + 7) / 8;
    let mut out = vec![0u8; out_len];
    for (out_pos, &m) in mapping.iter().enumerate() {
        if m <= 0 { continue; }
        let src_idx = (m as usize) - 1; // MSB zero-based
        let bit = get_bit_msb(input, src_idx);
        set_bit_msb(&mut out, out_pos, bit);
    }
    out
}

fn rotate_left_msb_bits(src: &[u8], total_bits: usize, shift: usize) -> Vec<u8> {
    let total_bits = total_bits.min(src.len() * 8);
    if total_bits == 0 { return vec![0u8; src.len()]; }
    let rot = shift % total_bits;
    let out_len = (total_bits + 7) / 8;
    let mut out = vec![0u8; out_len];
    for i in 0..total_bits {
        let src_pos = (i + rot) % total_bits;
        if get_bit_msb(src, src_pos) {
            set_bit_msb(&mut out, i, true);
        }
    }
    out
}

fn main() {
    // canonical test key
    let key_hex = "133457799BBCDFF1";
    let key_bytes = Vec::<u8>::from_hex(key_hex).expect("hex");

    // apply PC-1 (MSB path) to produce 56-bit (7-byte) buffer
    let pc1_out = permute_msb(&key_bytes, &PC_1);

    // split into left/right 28-bit halves (MSB indexing: bits 0..27 and 28..55)
    let mut left = vec![0u8; 4];
    let mut right = vec![0u8; 4];
    for i in 0..28 {
        if get_bit_msb(&pc1_out, i) { set_bit_msb(&mut left, i, true); }
    }
    for i in 0..28 {
        if get_bit_msb(&pc1_out, 28 + i) { set_bit_msb(&mut right, i, true); }
    }

    // rotation schedule
    let shifts = [1,1,2,2,2,2,2,2,1,2,2,2,2,2,2,1];

    println!("Reference (MSB) round keys for key {}:", key_hex);
    let mut cur_left = left;
    let mut cur_right = right;
    for (i, &s) in shifts.iter().enumerate() {
        cur_left = rotate_left_msb_bits(&cur_left, 28, s);
        cur_right = rotate_left_msb_bits(&cur_right, 28, s);

        // combine into 56-bit buffer
        let mut combined = vec![0u8; 7];
        for b in 0..28 {
            if get_bit_msb(&cur_left, b) { set_bit_msb(&mut combined, b, true); }
        }
        for b in 0..28 {
            if get_bit_msb(&cur_right, b) { set_bit_msb(&mut combined, 28 + b, true); }
        }

        let round_key = permute_msb(&combined, &PC_2);
        // print as hex
        println!("K{}: {}", i+1, hex::encode(round_key));
    }
}
