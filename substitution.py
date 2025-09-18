from help import load_file, strip_non_letters, write_to_file, single, digram_, trigram;

def substitution(i:int, stripped_cipher:str, cipher:str)->str:
    substitution_map = {
        'A': 'o', 'B': 'z', 'C': 'p', 'D': 'h', 'E': 'n', 'F': 'g', 'G': 'j', 'H': 'l', 
        'I': 'v', 'J': 'x', 'K': 'r', 'L': 's', 'M': 'c', 'N': 'q', 'O': 'e', 'P': 'y', 
        'Q': 'a', 'R': 'i', 'S': 'w', 'T': 'f', 'U': 't', 'V': 'd', 'W': 'm', 
        'X': 'u', 'Y': 'k', 'Z': 'b'  
    }
    decrypted_text = ""
    for char in cipher:
        if char.isalpha():
            decrypted_text += char if substitution_map.get(char)==""  else substitution_map.get(char)
        else:
            decrypted_text += char
    write_to_file(f"./substitution{i}.txt", decrypted_text)
    return decrypted_text
    



def main():
    i=6
    cipher = load_file(f"./cipher{i}.txt")
    stripped_cipher = strip_non_letters(cipher)
    single_map = single(stripped_cipher)
    digram_map = digram_(stripped_cipher)
    trigram_map = trigram(stripped_cipher)
    print(single_map, "\n", digram_map, "\n", trigram_map ,"\n")
    substitution(i, stripped_cipher, cipher)
    print(cipher)

if __name__ == "__main__":
    main()