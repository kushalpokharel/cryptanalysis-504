
def to_lowercase(s: str) -> str:
    return s.lower()

if __name__ == "__main__":
    read_from = "hill_with_key5.txt"
    write_to = "hill_lower.txt"
    data = open(read_from, 'r').read()
    lowercased_data = to_lowercase(data)
    open(write_to, 'w').write(lowercased_data)
    print(f"Lowercased data written to {write_to}")
