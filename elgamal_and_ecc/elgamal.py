def read_cipher(filename: str) -> list:
    with open(filename, 'r') as file:
        lines = file.readlines()
    ciphertexts = []
    for line in lines:
        tuple_str = ""
        for part in line:
            if( part == "("):
            # start of a tuple
                pass 
            elif part == ")":
                # end of a tuple
                if tuple_str:
                    c1, c2 = map(int, tuple_str.split(','))
                    ciphertexts.append((c1, c2))
                tuple_str = ""
            else:
                tuple_str += part
    return ciphertexts


def decrypt_elgamal(p: int, alpha: int, a: int, beta: int, ciphertexts: list) -> None:

    with open("elgamal_decrypted.txt", 'w') as file:
        for (c1, c2) in ciphertexts:
            power = 2 # since all the plaintext is 3 letters long
            s = pow(c1, a, p)
            s_inv = pow(s, p - 2, p)  # Modular inverse of s mod p when p is prime.
            assert((s * s_inv) % p == 1)
            m = (c2 * s_inv) % p
            print(f"Decrypted message: {m}")
            plaintext = ""
            flag = False
            while power >=0 :

                divisor = pow(26, power)
                quotient = m // divisor
                remainder = m % divisor
                character = chr(int(quotient) + ord('a'))
                print(character)
               
                plaintext += character
                power -=1
                m = remainder
            print(f"Plaintext: {plaintext}")
            file.write(plaintext + "\n")

def main():
    p =31847
    alpha = 5
    a = 7899
    beta = 18074
    ciphertexts = read_cipher("elgamal_cipher.txt")
    decrypt_elgamal(p, alpha, a, beta, ciphertexts)
    

if __name__ == "__main__":
    main()