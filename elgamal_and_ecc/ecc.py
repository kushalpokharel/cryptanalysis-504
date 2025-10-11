def generate_points_on_curve_brute_force(elliptic_curve_params:tuple[int,int, int]) -> list[tuple[int,int]]:
    a, b, p = elliptic_curve_params
    points = []
    for x in range(p):
        rhs = (x**3 + a*x + b) % p
        for y in range(p):
            if (y**2 % p) == rhs:
                points.append((x, y))
    # print("Points on the curve using brute force method:")
    # for point in points:
    #     print(point)
    return points

def generate_points_on_curve_quadratic_residue(elliptic_curve_params:tuple[int,int, int])-> list[tuple[int,int]]:
    a, b, p = elliptic_curve_params
    # since the p is a prime number and p%4=3, we can use the property of quadratic residue as:
    # if n is a quadratic residue mod p, then one of its square roots is n^((p+1)/4) mod p
    points = []
    for x in range(p):
        rhs = (x**3 + a*x + b) % p
        y_squared = rhs % p
        # check if y_squared is a quadratic residue mod p using Euler's criterion
        if pow(y_squared, (p - 1) // 2, p) == 1:
            y = pow(y_squared, (p + 1) // 4, p)
            points.append((x, y))
            if y != 0:
                points.append((x, p - y))  # add the negative root 
    # print("Points on the curve using quadratic residue method:")
    # for point in points:
    #     print(point)
    return points

def add_points(P:tuple[int,int], Q:tuple[int,int], elliptic_curve_params:tuple[int,int, int])->tuple[int,int]:
    a, b, p = elliptic_curve_params
    if P != Q:
        if P[0] == Q[0]:
            return (0, 0) # point at infinity
        m = (Q[1] - P[1]) * pow(Q[0] - P[0], p - 2, p) % p #(y2-y1)/(x2-x1) - slope of the intersecting line
    else:
        if P[1] == 0:
            return (0, 0)
        m = (3 * P[0]**2 + a) * pow(2 * P[1], p - 2, p) % p #(3x1^2 + a)/(2y1) - slope of the tangent line
    x_r = (m**2 - P[0] - Q[0]) % p
    y_r = (m * (P[0] - x_r) - P[1]) % p
    return (x_r, y_r)

def find_secret_key(elliptic_curve_params:tuple[int,int, int], alpha:tuple[int,int], beta:tuple[int,int], points:list[tuple[int,int]])->int:
    # brute force search for the secret key
    R = (0, 0)
    points_set = set()
    for d in range(1, len(points)+1):
        if d == 1:
            points_set.add(alpha)
            R = alpha
            continue
        R = add_points(R, alpha, elliptic_curve_params)
        # R should be a valid point on the curve and should not already be in the points set
        # since alpha is a generator point
        assert(R in points and R not in points_set) 
        points_set.add(R)

        # input("Press Enter to continue...")

        # print(f"alpha {alpha}, R {R}, d {d}")
        if R == beta:
            print(f"Secret key found: {d}")
            return d
    print("Secret key not found")
    return -1

def encrypt_elgamal(elliptic_curve_params:tuple[int,int, int], alpha:tuple[int,int], k:int, beta:tuple[int,int], message_point:tuple[int,int])->tuple[tuple[int,int], tuple[int,int]]:
    # C1 = k*alpha
    C1 = alpha
    for _ in range(2,k+1):
        C1 = add_points(C1, alpha, elliptic_curve_params)
    # C2 = M + k*beta
    C2 = beta
    for _ in range(2,k+1):
        C2 = add_points(C2, beta, elliptic_curve_params)
    C2 = add_points(message_point, C2, elliptic_curve_params)
    print(f"Ciphertext: (C1: {C1}, C2: {C2})")
    return (C1, C2)

def decrypt_elgamal(elliptic_curve_params:tuple[int,int, int], secret_key:int, cipher_text:tuple[tuple[int,int]])->tuple[int,int]:
    C1, C2 = cipher_text
    # S = d*C1
    S = C1
    for _ in range(2, secret_key+1):
        S = add_points(S, C1, elliptic_curve_params)
    # M = C2 - S
    S_neg = (S[0], (-S[1]) % elliptic_curve_params[2]) # negative of point S
    M = add_points(C2, S_neg, elliptic_curve_params)
    print(f"Decrypted message point: {M}")
    return M

def main():
    elliptic_curve_a = 1
    elliptic_curve_b = 6
    elliptic_curve_n = 1039
    elliptic_curve_params = (elliptic_curve_a, elliptic_curve_b, elliptic_curve_n)
    points_method_1 = generate_points_on_curve_brute_force(elliptic_curve_params)
    points_method_2 = generate_points_on_curve_quadratic_residue(elliptic_curve_params)

    assert(len(points_method_1) == len(points_method_2))
    assert(set(points_method_1) == set(points_method_2))
    print(f"Total number of points on the curve: {len(points_method_1)}")

    ## elgamal encryption parameters
    alpha = (799,790) # generator point
    k = 100 # random integer for elgamal encryption
    beta = (385, 749) # public key
    message_point = (575, 419) # message point to be encrypted
    secret_key = find_secret_key(elliptic_curve_params, alpha, beta, points_method_1)
    cipher_text = encrypt_elgamal(elliptic_curve_params, alpha, k, beta, message_point)

    ## test for encryption and decryption
    pt = decrypt_elgamal(elliptic_curve_params, secret_key, cipher_text)
    assert(pt == message_point)

    cipher_to_be_decrypted = ((873,233), (234,14))

    plain_text = decrypt_elgamal(elliptic_curve_params, secret_key, cipher_to_be_decrypted)
    print(f"Plaintext point: {plain_text}")

    # ## let's also find what k was used to encrypt the above cipher text
    # key = find_secret_key(elliptic_curve_params, alpha, cipher_to_be_decrypted[0], points_method_1)
    # print(f"Key used for encryption: {key}")

    ## Looks like k=100 was used to encrypt the above cipher text as well.
    ## let's verify by encrypting the same plaintext point with k=100
    cipher_test = encrypt_elgamal(elliptic_curve_params, alpha, 100, beta, plain_text)
    assert(cipher_test == cipher_to_be_decrypted)

    ## Last question: To find shared key between Alice and Bob
    generator = (818, 121)
    alice_public_key = (199,72)
    bob_public_key = (815,519)

    # we need to find one of Alice or Bob's secret key
    alice_secret_key = find_secret_key(elliptic_curve_params, generator, alice_public_key, points_method_1) 
    print(f"Alice's secret key: {alice_secret_key}")  

    # we can multiply Alice's secret key with Bob's public key to get the shared secret key
    shared_key = bob_public_key
    for _ in range(2, alice_secret_key+1):
        shared_key = add_points(shared_key, bob_public_key, elliptic_curve_params) 
    print(f"Shared key: {shared_key}")

    ## Just to verify, let's find Bob's secret key and multiply it with Alice's public key to get the shared key
    bob_secret_key = find_secret_key(elliptic_curve_params, generator, bob_public_key, points_method_1)
    print(f"Bob's secret key: {bob_secret_key}")
    shared_key_2 = alice_public_key
    for _ in range(2, bob_secret_key+1):
        shared_key_2 = add_points(shared_key_2, alice_public_key, elliptic_curve_params)
    print(f"Shared key 2: {shared_key_2}")
    assert(shared_key == shared_key_2)
    

if __name__ == "__main__":
    main()
