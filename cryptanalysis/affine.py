from cryptanalysis.help import load_file, append_to_file, strip_non_letters;

def get_ic(plaintext:str, a:int, b:int)->None:
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
        print(f"a={a}, b={b}")
        print("========================")
        input("Press Enter to continue...")


def affine(i:int, stripped_cipher:str):
    a_possible = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25]
    b_possible = list(range(26))
    for a in a_possible:
        for b in b_possible:
            decrypted = ""
            stripped_decrypted = ""
            for char in stripped_cipher:
                if char.isalpha():
                    y = ord(char) - ord('A')
                    stripped_decrypted += chr(((a * (y-b)) % 26) + ord('A'))
                    decrypted += chr(((a * (y-b)) % 26) + ord('a'))
                else:
                    decrypted += char
            print(f"a={a}, b={b}: {decrypted}")
            get_ic(stripped_decrypted, a, b)
            append_to_file(f"affine_decrypted{i}", f"{a}, {b}, {decrypted}\r\n")


def main():
    i=1
    cipher = load_file(f"./cipher{i}.txt")
    affine(i, cipher)
    print(cipher)

if __name__ == "__main__":
    main()