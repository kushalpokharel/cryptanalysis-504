from help import load_file, strip_non_letters, append_to_file, modMatInv;
import numpy as np
import math
from typing import Generator


def combination_of_4(n:int)->Generator:
    def _helper(): 
        for i in range(n):
            for j in range(n):
                for k in range(n):
                    for l in range(n):
                        yield [i,j,k,l]
        
    yield from _helper()

def combination_of_2(n:int)->Generator:
    def _helper(): 
        for i in range(n):
            for j in range(n):
                yield [i,j]
        
    yield from _helper()

def combination_of_3(n:int)->Generator:
    def _helper(): 
        for i in range(n):
            for j in range(n):
                for k in range(n):
                    yield [i,j,k]
        
    yield from _helper()

def get_ic(plaintext:str)->None:
    strip_non_letters(plaintext)
    prob = [0.082, 0.015, 0.028, 0.043, 0.127, 0.022, 0.020, 0.061, 0.070, 0.002, 0.008, 0.040, 0.024, 0.067, 0.075, 0.019, 0.001, 0.060, 0.063, 0.091, 0.028, 0.010, 0.024, 0.001, 0.020, 0.001]
    freq = {}
    for j in range(len(plaintext)):
        char = plaintext[j]
        if char in freq:
            freq[char] += 1
        elif char.isalpha():
            freq[char] = 1
        else: 
            continue
    n = sum(freq.values())
    ic = sum(v * prob[ord(k)-ord('A')] for (k, v) in freq.items()) / n if n > 1 else 0
    print(ic)
    global best_ic
    if(ic>0.058):
        best_ic = ic
        print (f"Ic: {ic}")
        print(plaintext)
        print("========================")
        input("Press Enter to continue...")


def lsfr_brute_force(i:int, stripped_cipher:str, cipher:str)->str:
    combinations = combination_of_4(26)
    key = "FRHFBPTL"
    for c in combinations:
        ## for the key with the linear combination of previous k keys with this coefficients
        key = c
        key_length = len(key)
        
        
# this approach would make the key same as with the vigenere approach 
# it is mentioned that the keys are different.
def lsfr_with_key(i:int, stripped_cipher:str, cipher:str, plain_text:str)->str:
    key_length = 2
    cipher_text = "KFYYITELJQQJ"
    cipher_text_m = [ord(c)-ord("A") for c in cipher_text[:2*key_length] if c.isalpha()]
    plain_text_m = [ord(c)-ord("A") for c in plain_text[:2*key_length]  if c.isalpha()]
    key = [ (cipher_text_m[j]-plain_text_m[j]+26) % 26 for j in range(len(plain_text_m))]
    # print("Key: ", key)
    # key = [ord(c)-ord('A') for c in "FRHFBPTL"]

    relevant_keys = key[:key_length*2]
    key_matrix = np.empty((key_length, key_length))
    print("Relevant Keys: ", relevant_keys)
    #construct the key matrix with first key_length*2 elements in 1..n 2..n+1 3..n+2 ... key_length..key_length*2
    for i in range(key_length):
        for j in range(key_length):
            key_matrix[i][j] = relevant_keys[i+j]
    
    print(key_matrix)

    # det = int(round(np.linalg.det(key_matrix))) % 26
    # print("Determinant: ", int(round(np.linalg.det(key_matrix))), det)
    # if(math.gcd(det,26) != 1):
    #     return

    print("Key Matrix: ", key_matrix)
    # get the coefficients of the linear combination by inverting the key matrix
    second_key_matrix = np.array(relevant_keys[key_length:]).reshape(1,key_length)
    print("Second Key Matrix: ", second_key_matrix)
    try:
        inv = modMatInv(key_matrix,26)
    except:
        print("Matrix not invertible")
        return
    print("Inverse Key Matrix: ", inv)
    # coefficient = np.array([20,19]).reshape(key_length,1)
    coefficient = (second_key_matrix @ inv) % 26
    print("Coefficients: ", coefficient)

    coefficient = coefficient.flatten().tolist()
    # get the rest of the key using the coefficients
    for i in range(key_length*2, len(cipher)):
        next_key = sum(coefficient[j] * key[i - key_length + j] for j in range(key_length)) % 26
        key.append(next_key)
    key_str = ''.join([chr(round(k) + ord('A')) for k in key])
    print("Full Key: ", key_str)
    #decrypt the cipher using the key
    decrypted_text = ""
    k=0
    for i in range(len(cipher)):
        if cipher[i].isalpha():
            decrypted_text += chr((ord(cipher[i]) - ord('A') - round(key[k]) + 26) % 26 + ord('A'))
            k+=1
        else:
            decrypted_text += cipher[i]
    get_ic(decrypted_text)
    # print("Decrypted text \n ", decrypted_text)
    # append_to_file(f"./lsfr_random.txt", f"LSFR Decryption with Key Matrix: {plain_text} and {key}\r\n")
    # append_to_file(f"./lsfr_random.txt", f"{decrypted_text}\r\n")



def main():
    i=2
    cipher = load_file(f"./cipher{i}.txt")
    # test()
    stripped_cipher = strip_non_letters(cipher)
    # for j in combination_of_4(26):
    #     print(j)
    #     plaintext = "".join([chr(k + ord('A')) for k in j])
    #     print(plaintext)
    #     lsfr_with_key(i, stripped_cipher, cipher, plaintext)
    lsfr_with_key(i, stripped_cipher, cipher, plain_text="SORTHELASTTWENTYYEARS")
    
    print(cipher)

if __name__ == "__main__":
    main()