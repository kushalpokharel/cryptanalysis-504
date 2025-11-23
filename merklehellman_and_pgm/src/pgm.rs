use std::{collections::HashMap, hash::Hash, task::ready, vec};

use curv::BigInt;

//// Given: size of r_i is from 10 to 3. i.e 8 logarithmic signature groups.
/// generate messages/numbers accordingly. 

fn read_logs(filename: &str) -> (Vec<Vec<Vec<usize>>>, Vec<Vec<usize>>){
    let contents = std::fs::read_to_string(filename).expect("Could not read file");
    let lines: Vec<&str> = contents.lines().collect();
    // group the lines into 10, 9, 8,..3 permutations (size of signature groups(sub))
    let mut row_permutations: Vec<Vec<Vec<usize>>> = Vec::new();
    let mut current_permutation: Vec<Vec<usize>> = Vec::new();
    let mut multiplier = 1;
    let mut row_values:Vec<Vec<usize>> = Vec::new();
    let mut prev_value = 0;
    let mut current_row_values:Vec<usize> = Vec::new();

    for (idx, line) in lines.iter().enumerate(){
        let parts: Vec<&str> = line.split_whitespace().collect();
        let curr = parts.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        current_permutation.push(curr);
        let row_value = (idx-prev_value)*multiplier;
        current_row_values.push(row_value);
        if idx==9 || idx==18 || idx==26 || idx==33 || idx==39 || idx==44 || idx==48 || idx==51{
            // println!("Multiplier before updating: {} {}", multiplier, prev_value);
            multiplier = multiplier*(idx-prev_value+1);
            prev_value = idx+1;
            row_permutations.push(current_permutation);
            row_values.push(current_row_values);
            current_permutation = Vec::new();
            current_row_values = Vec::new();
        }
        // println!("Line {}: {:?} : {:?}", idx + 1, parts, current_permutation);
    }
    // println!("Row values: {:?}", row_values);
    return (row_permutations, row_values);
}

fn read_cipher(filename: &str) -> String{
    let contents = std::fs::read_to_string(filename).expect("Could not read file");
    let lines: String = (contents.lines().collect::<Vec<&str>>().into_iter().map(|s| s.to_owned()).collect::<Vec<String>>()).join("");
    return lines;
}

fn get_alpha_indices(plaintext: &BigInt, permutations: &Vec<Vec<usize>>) -> Vec<usize>{
    let mut alpha_indices:Vec<usize> = Vec::new();
    let mut remaining = plaintext.clone();
    for (_, perm) in permutations.iter().rev().enumerate(){
        // println!("permutation: {:?}", perm);
        for (idx, row) in perm.iter().enumerate().rev(){
            // println!("Remaining: {} Row value: {}", remaining, row);
            // println!("permute: {:?} {}", row, idx);

            if remaining >= BigInt::from(row.clone() as u32){
                remaining = &remaining - BigInt::from(row.clone() as u32);
                alpha_indices.push(idx);
                break;
            }
            else if remaining < BigInt::from(row.clone() as u32) && idx==0{
                alpha_indices.push(idx);
                
            }
        }
    }
    // println!("Final remaining: {}", remaining);
    // the message should be exactly matched.
    assert!(remaining==BigInt::from(0));
    assert!(alpha_indices.len()==permutations.len());
    alpha_indices.reverse();
    // println!("Alpha indices: {:?}", alpha_indices);
    return alpha_indices;
}

fn combine_alpha_permutation(permutations: &Vec<Vec<Vec<usize>>>, alpha_indices: &Vec<usize>) -> Vec<usize>{
    let mut combined_permutation:Vec<usize> = Vec::new();
    for pos in 1..11{
        let mut current_pos = pos;
        for (i, alpha_index) in alpha_indices.iter().enumerate().rev(){
            //apply permutation in reverse order
            // println!("Applying permutation {} at index {} on position {}: ", i, alpha_index, current_pos);
            current_pos = permutations[i][*alpha_index][current_pos-1];
        }
        combined_permutation.push(current_pos);
        // println!("Position {} mapped to {}", pos, current_pos);
    }
    // each position should be unique
    return combined_permutation;
}

fn join_two_permutation(perm1: &Vec<usize>, perm2: &Vec<usize>) -> Vec<usize>{
    let mut joined_permutation:Vec<usize> = Vec::new();
    for pos in 1..11{
        let mapped_pos = perm2[perm1[pos-1]-1];
        joined_permutation.push(mapped_pos);
    }
    return joined_permutation;
}   

fn get_inverse_permutation(perm: &Vec<usize>) -> Vec<usize>{
    let mut inverse_perm:Vec<usize> = vec![0; perm.len()];
    for (i, val) in perm.iter().enumerate(){
        inverse_perm[*val - 1] = i + 1;
    }
    return inverse_perm;
}

fn get_beta_indices_from_combined_permutation(given_combined_permutation: Vec<usize>, beta_values: &Vec<Vec<Vec<usize>>>)->Vec<usize>{
    // match the permutation with beta values to get the index
    // multiply the inverse of this permutation with combined permutation to get the new permutation, the repeat.
    let mut combined_permutation = given_combined_permutation;
    let mut beta_indices:Vec<usize> = Vec::new();
    for (i, beta_perm) in beta_values.iter().enumerate(){
        let mut longest_match = 0;
        let mut best_index = 0;
        for (idx, beta_row) in beta_perm.iter().enumerate(){
            let mut match_count = 0;
            for (j, val) in beta_row.iter().enumerate(){
                if combined_permutation[j]==*val{
                    match_count +=1;
                }
                else{
                    break;
                }
            }
            if match_count > longest_match{
                longest_match = match_count;
                best_index = idx;
            }
        }
        beta_indices.push(best_index);
        // println!("Beta permutation {} matched longest at row with count {} at index {}", i, longest_match, best_index);
        let inverse = get_inverse_permutation(&beta_perm[best_index]);
        // println!("Inverse permutation {:?}\n: {:?}", &beta_perm[best_index] , inverse);

        combined_permutation = join_two_permutation(&combined_permutation, &inverse);
        // last ma sabai match huna parchha, also last ma combined_permutation should be identity
        if i==beta_values.len()-1{
            assert!(longest_match==10);
        }
        // println!("New combined permutation after beta {}: {:?}", i, combined_permutation);
    }
    println!("Final_combined_permutation: {:?}", combined_permutation);
    assert!(combined_permutation == (1..=10).collect::<Vec<usize>>());
    return beta_indices;

}

fn get_encrypted_value_from_beta(beta_indices: &Vec<usize>, messages: &Vec<Vec<usize>>) -> BigInt{
    let mut encrypted_value = BigInt::from(0);
    for (i, beta_index) in beta_indices.iter().enumerate(){
        let row_value = messages[i][*beta_index];
        // println!("Beta index {}: row value {}", i, row_value);
        encrypted_value = &encrypted_value + BigInt::from(row_value as u32);
    }
    println!("Final encrypted value: {}", encrypted_value);
    return encrypted_value;
}

fn encrypt_pgm(plaintext: &BigInt, alphalogs: &Vec<Vec<Vec<usize>>>, messages: &Vec<Vec<usize>>, betalogs: &Vec<Vec<Vec<usize>>>) -> BigInt{
    let MODULUS:BigInt = BigInt::from(857375);

    println!("Encrypting plaintext: {}", plaintext);
    // for alpha, get the indices using the superincreasing array of alpha_values.
    let alpha_indices = get_alpha_indices(plaintext, &messages);
    println!("Alpha indices: {:?}", alpha_indices);
    // combine the permutations of alpha to get the overall permutation
    let combined_permutation = combine_alpha_permutation(&alphalogs, &alpha_indices);
    // for beta, get the permutation one by one by matching with the combined_permuation and 
    // update the combined_permutation accordingly.
    let beta_indices = get_beta_indices_from_combined_permutation(combined_permutation, &betalogs);
    println!("Alpha indices: {:?}\nBeta indices: {:?}", alpha_indices, beta_indices);
    let encrypted = get_encrypted_value_from_beta(&beta_indices, &messages);
    return encrypted;
}

pub fn construct_map_for_cipher(matrix: &[&str;10])-> HashMap<char, usize>{
    let mut char_to_position:HashMap<char, usize> = std::collections::HashMap::new();
    let mut characters = 0;
    for (i, row) in matrix.iter().enumerate(){
        println!("Row {}: {}", i, row.len());
        for (j, ch) in row.chars().enumerate(){
            char_to_position.insert(ch, i*10+j);
            characters +=1;
        }
    }
    println!("{}",characters);
    // assert 95 charactrs read.
    assert!(characters==95);
    return char_to_position;
}

pub fn decrypt_pgm(){
    let (alpha_vec, values) = read_logs("/Users/kushalpokharel/Documents/Cryptography/merklehellman_and_pgm/cipher_pgm/log_alpha.txt");
    let (beta_vec, _) = read_logs("/Users/kushalpokharel/Documents/Cryptography/merklehellman_and_pgm/cipher_pgm/log_beta.txt");
    let ciphertext = read_cipher("/Users/kushalpokharel/Documents/Cryptography/merklehellman_and_pgm/cipher_pgm/PGM_ciphertext-2025.txt");
    println!("Read log_alpha.txt: {:?}", alpha_vec);
    // println!("Read log_beta.txt: {:?}", beta_vec);
    println!("Read permutation.txt: {:?}", values);
    // println!("Read ciphertext.txt: {:?}", ciphertext);
    let MODULUS:BigInt = BigInt::from(857375);

    let cipher_matrix =   [" !\"#$%&'()",
                                    "*+,-./0123",
                                    "456789:;<=",
                                    ">?@ABCDEFG",
                                    "HIJKLMNOPQ",
                                    "RSTUVWXYZ[",
                                    "\\]^_`abcde",
                                    "fghijklmno",
                                    "pqrstuvwxy",
                                    "z{|}~"];
    let positions = construct_map_for_cipher(&cipher_matrix);

    // read ciphertext in block of 3 characters and convert to number to get ciphetext, get key from PGM 
    // finally subtract the one time pad key from the ciphertext to get the plaintext number.
    // convert the plaintext number to characters again.
    let vals = vec![BigInt::from(95*95), BigInt::from(95), BigInt::from(1)];
    let mut full_plaintext = String::new();
    for i in 0..ciphertext.len()/3{
        let k = i as i32;
        let block = &ciphertext[i*3..(i+1)*3];
        let key_plaintext =  BigInt::from((1000+k) as u32);
        let mut ciphertext = BigInt::from(0);
        for (j, ch) in block.chars().enumerate(){
            let pos = positions.get(&ch).unwrap();
            ciphertext += &vals[j] * BigInt::from(*pos as u32);
        }
        println!("Ciphertext block {}: {}", block, ciphertext);
        let k_i = encrypt_pgm(&key_plaintext, &alpha_vec, &values, &beta_vec) % &MODULUS;
        println!("For key plaintext {} ciphertext {}: Encrypted key value: {}", key_plaintext, ciphertext, k_i);
        let x_i= (&ciphertext + &MODULUS  - &k_i ) % &MODULUS;
        println!("For ciphetext block {}: Decrypted value: {}", block, x_i);

        let mut x = x_i.clone();
        for (j, val) in vals.iter().enumerate(){
            let index = (&x)/val;
            println!("Index at position {}: {} {}", j, index, &x);
            positions.iter().for_each(|(ch, pos)| {
                if BigInt::from(*pos as u32) == index {
                    full_plaintext.push(*ch);
                    print!("{}", *ch);
                }
            });
            println!("");
            x = &x % val;
        }
    }
    println!("Full decrypted plaintext: {}", full_plaintext);



    // for i in 0..ciphertext.len(){
    //     // let plaintext = BigInt::from_str_radix(&ciphertext[i], 10).unwrap();
    //     // divide the plaintext into block of 3 letters
    //     let k = i as i32;
    //     for j in 0..ciphertext[i].len()/3{
    //         let block = &ciphertext[i][j*3..(j+1)*3];
    //         let ind:i32 = if k-1>0 {k-1} else {0};

    //         let plaintext =  BigInt::from((1000 + ind) as u32);

    //         let encrypted = encrypt_pgm(&plaintext, &alpha_vec, &values, &beta_vec);
    //         println!("For plaintext {}: Encrypted value: {}", plaintext, encrypted);
    //     }

    //     let encrypted = encrypt_pgm(&plaintext, &alpha_vec, &values, &beta_vec);
    //     println!("For plaintext {}: Encrypted value: {}", plaintext, encrypted);
    // }
    // let plaintext = BigInt::from((1003) as u32);
    // let encrypted = encrypt_pgm(&plaintext, &alpha_vec, &values, &beta_vec);   
    // println!("For plaintext {}: Encrypted value: {}", plaintext, encrypted);
}