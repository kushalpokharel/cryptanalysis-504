use hex::FromHex;

// Copy essential DES tables (E, IP, S, P, PC_1, PC_2) for standalone use
const E: [i32;48] = [
    32,1,2,3,4,5,4,5,6,7,8,9,8,9,10,11,12,13,12,13,14,15,16,17,16,17,18,19,20,21,20,21,22,23,24,25,24,25,26,27,28,29,28,29,30,31,32,1
];

const IP: [i32;64] = [
    58,50,42,34,26,18,10,2,60,52,44,36,28,20,12,4,62,54,46,38,30,22,14,6,64,56,48,40,32,24,16,8,57,49,41,33,25,17,9,1,59,51,43,35,27,19,11,3,61,53,45,37,29,21,13,5,63,55,47,39,31,23,15,7
];

const S: [[[u8;16];4];8] = [
    [[14,4,13,1,2,15,11,8,3,10,6,12,5,9,0,7],[0,15,7,4,14,2,13,1,10,6,12,11,9,5,3,8],[4,1,14,8,13,6,2,11,15,12,9,7,3,10,5,0],[15,12,8,2,4,9,1,7,5,11,3,14,10,0,6,13]],
    [[15,1,8,14,6,11,3,4,9,7,2,13,12,0,5,10],[3,13,4,7,15,2,8,14,12,0,1,10,6,9,11,5],[0,14,7,11,10,4,13,1,5,8,12,6,9,3,2,15],[13,8,10,1,3,15,4,2,11,6,7,12,0,5,14,9]],
    [[10,0,9,14,6,3,15,5,1,13,12,7,11,4,2,8],[13,7,0,9,3,4,6,10,2,8,5,14,12,11,15,1],[13,6,4,9,8,15,3,0,11,1,2,12,5,10,14,7],[1,10,13,0,6,9,8,7,4,15,14,3,11,5,2,12]],
    [[7,13,14,3,0,6,9,10,1,2,8,5,11,12,4,15],[13,8,11,5,6,15,0,3,4,7,2,12,1,10,14,9],[10,6,9,0,12,11,7,13,15,1,3,14,5,2,8,4],[3,15,0,6,10,1,13,8,9,4,5,11,12,7,2,14]],
    [[2,12,4,1,7,10,11,6,8,5,3,15,13,0,14,9],[14,11,2,12,4,7,13,1,5,0,15,10,3,9,8,6],[4,2,1,11,10,13,7,8,15,9,12,5,6,3,0,14],[11,8,12,7,1,14,2,13,6,15,0,9,10,4,5,3]],
    [[12,1,10,15,9,2,6,8,0,13,3,4,14,7,5,11],[10,15,4,2,7,12,9,5,6,1,13,14,0,11,3,8],[9,14,15,5,2,8,12,3,7,0,4,10,1,13,11,6],[4,3,2,12,9,5,15,10,11,14,1,7,6,0,8,13]],
    [[4,11,2,14,15,0,8,13,3,12,9,7,5,10,6,1],[13,0,11,7,4,9,1,10,14,3,5,12,2,15,8,6],[1,4,11,13,12,3,7,14,10,15,6,8,0,5,9,2],[6,11,13,8,1,4,10,7,9,5,0,15,14,2,3,12]],
    [[13,2,8,4,6,15,11,1,10,9,3,14,5,0,12,7],[1,15,13,8,10,3,7,4,12,5,6,11,0,14,9,2],[7,11,4,1,9,12,14,2,0,6,10,13,15,3,5,8],[2,1,14,7,4,10,8,13,15,12,9,0,3,5,6,11]]
];

const P: [i32;32] = [16,7,20,21,29,12,28,17,1,15,23,26,5,18,31,10,2,8,24,14,32,27,3,9,19,13,30,6,22,11,4,25];

const PC_1: [i32;56] = [57,49,41,33,25,17,9,1,58,50,42,34,26,18,10,2,59,51,43,35,27,19,11,3,60,52,44,36,63,55,47,39,31,23,15,7,62,54,46,38,30,22,14,6,61,53,45,37,29,21,13,5,28,20,12,4];

const PC_2: [i32;48] = [14,17,11,24,1,5,3,28,15,6,21,10,23,19,12,4,26,8,16,7,27,20,13,2,41,52,31,37,47,55,30,40,51,45,33,48,44,49,39,56,34,53,46,42,50,36,29,32];

fn get_bit_msb(slice: &[u8], bit_index: usize) -> bool {
    let byte_idx = bit_index / 8;
    if byte_idx >= slice.len() { return false; }
    let bit_in_byte = (bit_index % 8) as u8;
    let shift = 7u8 - bit_in_byte;
    ((slice[byte_idx] >> shift) & 1) != 0
}

fn set_bit_msb(slice: &mut [u8], bit_index: usize, value: bool) {
    let byte_idx = bit_index / 8;
    if byte_idx >= slice.len() { return; }
    let bit_in_byte = (bit_index % 8) as u8;
    let shift = 7u8 - bit_in_byte;
    if value { slice[byte_idx] |= 1 << shift; } else { slice[byte_idx] &= !(1 << shift); }
}

fn permute_msb(input: &[u8], mapping: &[i32]) -> Vec<u8> {
    let out_bits = mapping.len();
    let out_len = (out_bits + 7) / 8;
    let mut out = vec![0u8; out_len];
    for (out_pos, &m) in mapping.iter().enumerate() {
        if m <= 0 { continue; }
        let src_idx = (m as usize) - 1;
        let bit = get_bit_msb(input, src_idx);
        set_bit_msb(&mut out, out_pos, bit);
    }
    out
}

fn rotate_left_msb_bits(src: &[u8], total_bits: usize, shift: usize) -> Vec<u8> {
    let total_bits = total_bits.min(src.len() * 8);
    let rot = if total_bits==0 {0} else { shift % total_bits };
    let out_len = (total_bits + 7) / 8;
    let mut out = vec![0u8; out_len];
    for i in 0..total_bits {
        let src_pos = (i + rot) % total_bits;
        if get_bit_msb(src, src_pos) { set_bit_msb(&mut out, i, true); }
    }
    out
}

// LSB helpers
fn get_bit_lsb(slice: &[u8], bit_index: usize) -> bool {
    let byte_idx = bit_index / 8;
    if byte_idx >= slice.len() { return false; }
    let bit_in_byte = bit_index % 8;
    ((slice[byte_idx] >> bit_in_byte) & 1) != 0
}

fn set_bit_lsb(slice: &mut [u8], bit_index: usize, value: bool) {
    let byte_idx = bit_index / 8;
    if byte_idx >= slice.len() { return; }
    let bit_in_byte = bit_index % 8;
    if value { slice[byte_idx] |= 1 << bit_in_byte; } else { slice[byte_idx] &= !(1 << bit_in_byte); }
}

fn permute_lsb(input: &[u8], mapping: &[i32]) -> Vec<u8> {
    // mapping is MSB 1-based; we need to read source bits using LSB indexing
    let out_bits = mapping.len();
    let out_len = (out_bits + 7) / 8;
    let mut out = vec![0u8; out_len];
    for (out_pos, &m) in mapping.iter().enumerate() {
        if m <= 0 { continue; }
        let msb_zero = (m as usize) - 1;
        let msb_byte = msb_zero / 8;
        let msb_bit_in_byte = msb_zero % 8;
        let lsb_index = msb_byte * 8 + (7 - msb_bit_in_byte);
        let bit = get_bit_lsb(input, lsb_index);
        // set out at out_pos (MSB-based) -> convert to LSB index in out
        let out_byte = out_pos / 8;
        let out_bit_in_byte = out_pos % 8;
        let out_lsb = out_byte * 8 + (7 - out_bit_in_byte);
        set_bit_lsb(&mut out, out_lsb, bit);
    }
    out
}

fn rotate_left_lsb(src: &[u8], total_bits: usize, shift: usize) -> Vec<u8> {
    let total_bits = total_bits.min(src.len() * 8);
    if total_bits == 0 { return vec![0u8; src.len()]; }
    let rot = shift % total_bits;
    let mut out = vec![0u8; src.len()];
    for i in 0..total_bits {
        let src_pos = (i + rot) % total_bits;
        if get_bit_lsb(src, src_pos) { set_bit_lsb(&mut out, i, true); }
    }
    out
}

fn groups_of_6_msb(src: &[u8], total_bits: usize) -> Vec<u8> {
    let total_bits = total_bits.min(src.len()*8);
    let full_groups = total_bits / 6;
    let mut out = Vec::with_capacity(full_groups);
    for g in 0..full_groups {
        let bit_index = g*6;
        let mut val = 0u8;
        for b in 0..6 {
            let global_bit = bit_index + b;
            let byte_idx = global_bit / 8;
            let bit_in_byte = global_bit % 8;
            let shift = 7 - (bit_in_byte as u8);
            let bit = if byte_idx < src.len() && ((src[byte_idx] >> shift) & 1) != 0 {1} else {0};
            val = (val << 1) | bit;
        }
        out.push(val);
    }
    out
}

fn groups_of_6_lsb(src: &[u8], total_bits: usize) -> Vec<u8> {
    let total_bits = total_bits.min(src.len() * 8);
    let full_groups = total_bits / 6;
    let mut out = Vec::with_capacity(full_groups);
    for g in 0..full_groups {
        let bit_index = g*6;
        let byte_idx = bit_index / 8;
        let shift = (bit_index % 8) as u16;
        let lo = src.get(byte_idx).copied().unwrap_or(0) as u16;
        let hi = src.get(byte_idx+1).copied().unwrap_or(0) as u16;
        let word = lo | (hi << 8);
        let val = ((word >> shift) & 0x3F) as u8;
        out.push(val);
    }
    out
}

fn pack_8_nibbles_to_4_bytes(nibbles: &[u8]) -> [u8;4] {
    let mut out = [0u8;4];
    for i in 0..4 {
        out[i] = (nibbles[2*i] & 0x0F) | ((nibbles[2*i+1] & 0x0F) << 4);
    }
    out
}

fn reverse_6bits(v:u8)->u8{
    (((v & 0x01) << 5) | ((v & 0x02) << 3) | ((v & 0x04) << 1) | ((v & 0x08) >> 1) | ((v & 0x10) >> 3) | ((v & 0x20) >> 5)) as u8
}

fn main(){
    let key_hex = "133457799BBCDFF1";
    let pt_hex = "0123456789ABCDEF";
    let key = Vec::<u8>::from_hex(key_hex).unwrap();
    let pt = Vec::<u8>::from_hex(pt_hex).unwrap();

    // MSB path: compute round 1 key
    let pc1 = permute_msb(&key, &PC_1);
    let (mut l, mut r) = (vec![0u8;4], vec![0u8;4]);
    for i in 0..28{ if get_bit_msb(&pc1, i) { set_bit_msb(&mut l, i, true);} }
    for i in 0..28{ if get_bit_msb(&pc1, 28+i) { set_bit_msb(&mut r, i, true);} }
    l = rotate_left_msb_bits(&l, 28, 1);
    r = rotate_left_msb_bits(&r, 28, 1);
    let mut combined = vec![0u8;7];
    for b in 0..28 { if get_bit_msb(&l,b){ set_bit_msb(&mut combined,b,true); } }
    for b in 0..28 { if get_bit_msb(&r,b){ set_bit_msb(&mut combined,28+b,true); } }
    let round1_msb = permute_msb(&combined, &PC_2);

    // LSB path: compute round 1 key (LSB semantics)
    // For PC-1 we interpret mapping similarly but produce LSB-packed pc1
    // Build pc1_lsb by permute_lsb using PC_1
    let pc1_lsb = permute_lsb(&key, &PC_1);
    // split into halves (LSB indexing: first 28 bits are left)
    let mut left_l = vec![0u8;4];
    let mut right_l = vec![0u8;4];
    for i in 0..28 { if get_bit_lsb(&pc1_lsb, i) { set_bit_lsb(&mut left_l, i, true);} }
    for i in 0..28 { if get_bit_lsb(&pc1_lsb, 28+i) { set_bit_lsb(&mut right_l, i, true);} }
    left_l = rotate_left_lsb(&left_l, 28, 1);
    right_l = rotate_left_lsb(&right_l, 28, 1);
    let mut combined_l = vec![0u8;7];
    for b in 0..28 { if get_bit_lsb(&left_l,b){ set_bit_lsb(&mut combined_l,b,true); } }
    for b in 0..28 { if get_bit_lsb(&right_l,b){ set_bit_lsb(&mut combined_l,28+b,true); } }
    let round1_lsb = permute_lsb(&combined_l, &PC_2);

    println!("Round-1 MSB key: {}", hex::encode(&round1_msb));
    println!("Round-1 LSB key: {}", hex::encode(&round1_lsb));

    // Now compute right half of IP for both conventions
    let ip_msb = permute_msb(&pt, &IP);
    let right_msb = ip_msb[4..8].to_vec();

    let ip_lsb = permute_lsb(&pt, &IP);
    let right_lsb = ip_lsb[4..8].to_vec();

    println!("\n=== MSB f() diagnostics ===");
    // MSB f
    let e_msb = permute_msb(&right_msb, &E);
    println!("E-expanded MSB: {}", hex::encode(&e_msb));
    let after_xor_msb: Vec<u8> = e_msb.iter().zip(round1_msb.iter()).map(|(a,b)| a ^ b).collect();
    println!("After XOR MSB: {}", hex::encode(&after_xor_msb));
    let groups_msb = groups_of_6_msb(&after_xor_msb, 48);
    println!("6-bit groups MSB: {:?}", groups_msb);
    let mut s_outs_msb = vec![];
    for (i,&g) in groups_msb.iter().enumerate(){
        let col = ((g>>1)&0x0F) as usize;
        let row = ((((g>>5)&1)<<1)|(g&1)) as usize;
        let s = S[i][row][col];
        s_outs_msb.push(s);
        println!("grp {:02X} -> row {} col {} s_out {:02X}", g, row, col, s);
    }
    let packed_msb = pack_8_nibbles_to_4_bytes(&s_outs_msb);
    println!("Packed before P MSB: {}", hex::encode(packed_msb));
    let p_msb = permute_msb(&packed_msb, &P);
    println!("After P MSB: {}", hex::encode(p_msb));

    println!("\n=== LSB f() diagnostics ===");
    let e_lsb = permute_lsb(&right_lsb, &E);
    println!("E-expanded LSB: {}", hex::encode(&e_lsb));
    let after_xor_lsb: Vec<u8> = e_lsb.iter().zip(round1_lsb.iter()).map(|(a,b)| a ^ b).collect();
    println!("After XOR LSB: {}", hex::encode(&after_xor_lsb));
    let groups_lsb = groups_of_6_lsb(&after_xor_lsb, 48);
    println!("6-bit groups LSB (raw): {:?}", groups_lsb);
    let mut s_outs_lsb = vec![];
    for (i,&graw) in groups_lsb.iter().enumerate(){
        let g = reverse_6bits(graw);
        let col = ((g>>1)&0x0F) as usize;
        let row = ((((g>>5)&1)<<1)|(g&1)) as usize;
        let s = S[i][row][col];
        s_outs_lsb.push(s);
        println!("raw {:02X} rev {:02X} -> row {} col {} s_out {:02X}", graw, g, row, col, s);
    }
    let packed_lsb = pack_8_nibbles_to_4_bytes(&s_outs_lsb);
    println!("Packed before P LSB: {}", hex::encode(packed_lsb));
    let p_lsb = permute_lsb(&packed_lsb, &P);
    println!("After P LSB: {}", hex::encode(p_lsb));
}
