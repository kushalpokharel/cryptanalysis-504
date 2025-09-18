from help import load_file, strip_non_letters, append_to_file, modMatInv;
import numpy as np


# this approach would make the key same as with the vigenere approach 
# it is mentioned that the keys are different.
def lsfr_with_key(i:int, stripped_cipher:str, cipher:str)->str:
    key_length = 2
    cipher_text = "XZTUVDBHYK"
    plain_text = "FORTHELAST"
    cipher_text_m = [ord(c)-ord("A") for c in cipher_text[:2*key_length] if c.isalpha()]
    plain_text_m = [ord(c)-ord("A") for c in plain_text[:2*key_length]  if c.isalpha()]
    key = [ (cipher_text_m[j]-plain_text_m[j]+26) % 26 for j in range(len(plain_text_m))]
    print("Key: ", key)

    relevant_keys = key[:key_length*2]
    key_matrix = np.empty((key_length, key_length))
    #construct the key matrix with first key_length*2 elements in 1..n 2..n+1 3..n+2 ... key_length..key_length*2
    for i in range(key_length):
        for j in range(key_length):
            key_matrix[i][j] = relevant_keys[i+j]
    print("Key Matrix: ", key_matrix)
    # get the coefficients of the linear combination by inverting the key matrix
    second_key_matrix = np.array(relevant_keys[key_length:]).reshape(1,key_length)
    print("Second Key Matrix: ", second_key_matrix)
    inv = modMatInv(key_matrix,26)
    print("Inverse Key Matrix: ", inv)
    coefficient = (second_key_matrix @ inv) %26
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
    print("Decrypted text \n ", decrypted_text)
    # append_to_file(f"./hill_with_key{i}.txt", f"Hill Decryption with Key Matrix: {key_list}\r\n")
    # append_to_file(f"./hill_with_key{i}.txt", decrypted+"\r\n")




def main():
    i=2
    cipher = load_file(f"./cipher{i}.txt")
    # test()
    stripped_cipher = strip_non_letters(cipher)
    lsfr_with_key(i, stripped_cipher, cipher)
    print(cipher)

if __name__ == "__main__":
    main()