from cryptanalysis.help import load_file, strip_non_letters, append_to_file, single, digram_, trigram
from typing import Generator, List


def permute(n: int) -> Generator[List[int], None, None]:
    def _helper(current: List[int]):
        if len(current) == n:
            yield current.copy()
            return
        for i in range(n):
            if i in current:
                continue
            current.append(i)
            yield from _helper(current)
            current.pop()

    yield from _helper([])

def all_permutation(n:int, stripped_cipher:str, cipher:str)->str:
    key_length = 9
    permutation = permute(key_length)
    for p in permutation:
        decrypted_text = ""
        # check if it is a simple permutation from 0 to key_length-1 and continue if sos
        flag = True
        for i in range(0, key_length):
            if p[i] == i:
                continue
            else:
                flag = False
                break
        if flag:
            continue
        # print(p)
        for i in range(0, key_length*5, key_length):
            block = stripped_cipher[i:i+key_length]
            decrypted_block = [''] * key_length
            for j in range(0, key_length):
                decrypted_block[j] = block[p[j]]
            decrypted_text += ''.join(decrypted_block)
        
        # put the stripped chars back to their original positions in the cipher text
        decrypted = ""
        k = 0
        for char in cipher:
            if k >= len(decrypted_text):
                break
            if char.isalpha():
                decrypted += decrypted_text[k]
                k += 1
            else:
                decrypted += char
        append_to_file(f"./permutations{key_length}.txt", f"Permutation: {p}\r\n")
        append_to_file(f"./permutations{key_length}.txt", decrypted)
        append_to_file(f"./permutations{key_length}.txt",f"\r\n")
    return decrypted_text

def permutation_with_correct_key(n:int, stripped_cipher:str, cipher:str, key:List[int])->str:
    key_length = len(key)
    decrypted_text = ""
    print(key_length)
    for i in range(0, len(stripped_cipher)-key_length, key_length):
        block = stripped_cipher[i:i+key_length]
        decrypted_block = [''] * key_length
        for j in range(0, key_length):
            decrypted_block[j] = block[key[j]]
        decrypted_text += ''.join(decrypted_block)
    
    # put the stripped chars back to their original positions in the cipher text
    decrypted = ""
    k = 0
    for char in cipher:
        if k >= len(decrypted_text):
            break
        if char.isalpha():
            decrypted += decrypted_text[k]
            k += 1
        else:
            decrypted += char
    append_to_file(f"./permutation_with_key{key_length}.txt", f"Permutation: {key}\r\n")
    append_to_file(f"./permutation_with_key{key_length}.txt", decrypted)


def main():
    i=3
    cipher = load_file(f"./cipher{i}.txt")
    stripped_cipher = strip_non_letters(cipher)
    # if it is permutation then the individual letters occurrences will not change.
    single_map = single(stripped_cipher)
    # looks like 3 could be a permutation cipher. Let's test its index of coincidence.
    n = len(stripped_cipher)
    freq_sum = sum(v*(v-1) for v in single_map.values())
    ic = freq_sum / (n*(n-1)) 
    # with an ic of 0.064, it is likely a permutation cipher.

    print(f"Index of Coincidence for cipher {i}: {ic}")
    print(single_map, "\n")
    # all_permutation(i, stripped_cipher, cipher)
    # after getting the correct key by manual analysis of the outputs of all_permutation
    key = [7, 1, 8, 2, 6, 0, 5, 4, 3]
    permutation_with_correct_key(i, stripped_cipher, cipher, key)
    print(cipher)

if __name__ == "__main__":
    main()