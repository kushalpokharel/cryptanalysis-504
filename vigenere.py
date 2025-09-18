from help import load_file, strip_non_letters, append_to_file, single, digram_, trigram


def index_of_coincidence(text: str, probable_key_length: int) -> float:
    for i in range(probable_key_length):
        freq = {}
        for j in range(i, len(text), probable_key_length):
            char = text[j]
            if char in freq:
                freq[char] += 1
            else:
                freq[char] = 1
        n = sum(freq.values())
        ic = sum(count * (count - 1) for count in freq.values()) / (n * (n - 1)) if n > 1 else 0
        print (f"IC for segment {i}: {ic}")


def vigenere(n:int, stripped_cipher:str, cipher:str)->str:
    index_of_coincidence(stripped_cipher, 12)
    # looks like 12 is a probable key length. Let's try to decrypt it using frequency analysis.
    prob = [0.082, 0.015, 0.028, 0.043, 0.127, 0.022, 0.020, 0.061, 0.070, 0.002, 0.008, 0.040, 0.024, 0.067, 0.075, 0.019, 0.001, 0.060, 0.063, 0.091, 0.028, 0.010, 0.024, 0.001, 0.020, 0.001]
    # the above are the frequencies
    key_length = 12
    # key = "SLCBOZQHGRUT"
    key = ""
    freq = {}
    for i in range(key_length):
        print(i)
        freq = {}
        for j in range(i, len(stripped_cipher), key_length):
            char = stripped_cipher[j]
            if char in freq:
                freq[char] += 1
            else:
                freq[char] = 1
        max_ic = 0
        best_shift = 0
        for j in range(26):
            shifted_freq = {}
            for char in freq:
                shifted_char = chr((ord(char) - ord('A') - j + 26) % 26 + ord('A'))
                shifted_freq[shifted_char] = freq[char]
            n = sum(shifted_freq.values())
            ic = sum(shifted_freq.get(chr(k + ord('A')), 0) * prob[k] for k in range(26)) / n if n > 0 else 0
            print(f"Shift: {j}, IC: {ic}")
            
            if(ic>max_ic):
                print(f"Shift: {j}, IC: {ic}")
                max_ic = ic
                best_shift = j
        key+=chr(best_shift + ord('A'))
    
    k=0
    decrypted_text = ""
    for i in range(len(cipher)):
        if cipher[i].isalpha():
            decrypted_text += chr((ord(cipher[i]) - ord('A') - (ord(key[k % key_length]) - ord('A')) + 26) % 26 + ord('A'))
            k+=1
        else:
            decrypted_text += cipher[i]
    # print(decrypted_text)
    append_to_file(f"./vigenere{key_length}.txt", decrypted_text)
    print(f"Probable key: {key}")



def main():
    i=2
    cipher = load_file(f"./cipher{i}.txt")
    stripped_cipher = strip_non_letters(cipher)

    # with an ic of 0.064, it is likely a permutation cipher.

    vigenere(i, stripped_cipher, cipher)
    print(cipher)

if __name__ == "__main__":
    main()