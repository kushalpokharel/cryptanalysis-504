// struct Key{
//     bit_vector:Vec<bool>,
//     hex_string:String
// }

// impl Permutable for Key{
//     /// Permute bits from `src_bytes` according to `mapping` (1-based values).
//     /// Returns a new Vec<u8> containing packed bits (MSB-first).
//     fn permute_with_mapping(&mut self, mapping: Vec<i32>)->Key{
//         let out_bits = mapping.len();
//         let out_len = (out_bits + 7) / 8;
//         // allocate output buffer (initialized to zero)
//         let mut out: Vec<u8> = vec![0u8; out_len];

//         // debug: print a few diagnostics to help reason about zero output
//         for (out_pos, &m) in mapping.iter().enumerate() {
//             let src_bit_idx = (m as usize) - 1; // zero-based
//             let src_byte_idx = src_bit_idx / 8;
//             if src_byte_idx >= self.actual_keys.len() {
//                 eprintln!("Mapping index {} (src bit {}) out of bounds for input of {} bytes", m, src_bit_idx, self.actual_keys.len());
//                 continue;
//             }
//             let src_byte = self.actual_keys[src_byte_idx];
//             let bit_val = self.get_bit_msb(src_bit_idx);
//             // Print only the first 16 mapped bits to avoid huge output
//             // if out_pos < 16 {
//             //     println!("mapping[{}] = {} -> src_bit {} (byte {} = {:08b}) => bit_val={}", out_pos, m, src_bit_idx, src_byte_idx, src_byte, bit_val);
//             // }
//             self.set_bit_msb(&mut out, out_pos, bit_val);
//         }
//         Key{actual_keys:out}
//     }
//     fn permute_with_reverse_mapping(&mut self, mapping: Vec<i32>)->Key{
//         println!("Permutation output for mapping ");
//         let out_bits = mapping.len();
//         let out_len = (out_bits + 7) / 8;
//         // allocate output buffer (initialized to zero)
//         let mut out: Vec<u8> = vec![0u8; out_len];

//         // debug: print a few diagnostics to help reason about zero output
//         // mapping semantic (this variant): mapping[i] = destination (1-based) for the bit
//         // at source index i (0-based). Enumerate yields (src_index, &dest_one_based).
//         for (src_idx, &dest_one_based) in mapping.iter().enumerate() {
//             let src_bit_idx = src_idx; // zero-based source bit index
//             // ensure source bit exists
//             if src_bit_idx >= self.actual_keys.len() * 8 {
//                 eprintln!("Source bit index {} out of range for input of {} bytes", src_bit_idx, self.actual_keys.len());
//                 continue;
//             }
//             let bit_val = self.get_bit_msb(src_bit_idx);

//             // convert destination to zero-based and guard bounds for output
//             if dest_one_based <= 0 {
//                 eprintln!("Invalid destination mapping value {} at src {}", dest_one_based, src_idx);
//                 continue;
//             }
//             let dest_zero = (dest_one_based as usize) - 1;
//             if dest_zero >= out_bits {
//                 eprintln!("Destination {} (zero {}) out of range for output bits {}", dest_one_based, dest_zero, out_bits);
//                 continue;
//             }
//             self.set_bit_msb(&mut out, dest_zero, bit_val);
//         }
//         Key{actual_keys:out}
//     }
// }