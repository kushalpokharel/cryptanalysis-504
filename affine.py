from help import load_file, write_to_file;

def affine(i:int, stripped_cipher:str):
    a_possible = [1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25]
    b_possible = list(range(26))
    for a in a_possible:
        for b in b_possible:
            decrypted = ""
            for char in stripped_cipher:
                if char.isalpha():
                    x = ord(char) - ord('A')
                    decrypted += chr(((a * x + b) % 26) + ord('A'))
                else:
                    decrypted += char
            print(f"a={a}, b={b}: {decrypted}")
            write_to_file(f"affine_decrypted{i}", f"{a}, {b}, {decrypted}\r\n")



def main():
    i=1
    cipher = load_file(f"./cipher{i}.txt")
    affine(i, cipher)
    print(cipher)

if __name__ == "__main__":
    main()