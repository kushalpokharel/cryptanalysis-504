from cryptanalysis.help import load_file, strip_non_letters, append_to_file, modMatInv;
from typing import Generator, List
import numpy as np

best_ic = 0
best_plaintext = ""

def combination_of_4(n:int)->Generator:
    def _helper(): 
        for i in range(n):
            for j in range(n):
                for k in range(n):
                    for l in range(n):
                        yield [i,j,k,l]
        
    yield from _helper()


def gcd(a:int, b:int)->int:
    if a < b:
        a, b = b, a
    while b:
        a, b = b, a % b
    return a

def get_ic(plaintext:str)->None:
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


def hill(i:int, stripped_cipher:str, cipher:str, key:List=[[]])->str:
    comb = combination_of_4(26)
    if key:
        comb = key
    for c in comb:
        # create a numpy 2x2 matrix from the combination
        arr = np.array(c).reshape(2,2)
        decrypted_text = "Hill Decryption with key: "+str(c)+"\r\n"
        if(gcd(round(np.linalg.det(arr)), 26)!=1):
            continue
        inv = modMatInv(arr, 26)
        print(decrypted_text)
       
        decrypted_subtext = ""
        for j in range(0, len(stripped_cipher), 2):
            block = stripped_cipher[j:j+2]
            if len(block) < 2:
                break
            block_arr = np.array([ord(block[0]) - ord('A'), ord(block[1]) - ord('A')]).reshape(2,1)
            decrypted_block = inv @ block_arr % 26
            decrypted_block = ''.join([chr((round(float(x)%26) + ord('A'))) for x in decrypted_block])
            decrypted_subtext += decrypted_block
        k = 0
        get_ic(decrypted_subtext)
        for char in cipher:
            if char.isalpha():
                decrypted_text+=decrypted_subtext[k]
                k+=1
            else:
                decrypted_text+=char
        decrypted_text+="\r\n"
        append_to_file(f"./hill_with_key{i}.txt", f"Hill Decryption with Key Matrix: {key}\r\n")
        append_to_file(f"./hill_with_key{i}.txt", decrypted_text+"\r\n")


## 2 or 4 wasn't hill crypto. one of those vigenere ciphertext is also lsfr. 5 is hill
def hill_with_key(i:int, stripped_cipher:str, cipher:str)->str:
    cipher_text = "HIZL"
    plain_text = "MATH"
    cipher_text_m = [ord(c)-ord("A") for c in cipher_text if c.isalpha()]
    plain_text_m = [ord(c)-ord("A") for c in plain_text  if c.isalpha()]

    cipher_matrix = np.array(cipher_text_m).reshape(2,2)
    plain_matrix = np.array(plain_text_m).reshape(2,2)
    key_matrix = plain_matrix @ modMatInv(cipher_matrix, 26) % 26
    key_list = [round(x) for x in key_matrix.flatten()]
    print("Key Matrix: ", key_list)
    key_length = len(key_list)
    decrypted_text = ""
    for j in range(0, len(stripped_cipher), key_length):
        block = stripped_cipher[j:j+key_length]
        if len(block) < key_length:
            break
        block_arr = np.array([ord(block[k]) - ord('A') for k in range(key_length)]).reshape(2,2)
        decrypted_block = key_matrix @ block_arr % 26
        decrypted_block = ''.join([chr(round(x) + ord('A')) for x in decrypted_block.flatten()])
        decrypted_text += decrypted_block
    k = 0
    decrypted = ""
    for char in cipher:
        if k >= len(decrypted_text):
            break
        if char.isalpha():
            decrypted += decrypted_text[k]
            k += 1
        else:
            decrypted += char



def main():
    i=5
    cipher = load_file(f"./cipher{i}.txt")
    # test()
    stripped_cipher = strip_non_letters(cipher)
    ## we have the key after trying all combination and frequency analysis
    hill(i, stripped_cipher, cipher, [[10, 5, 19, 25]])
    print(cipher)

if __name__ == "__main__":
    main()