import numpy
from numpy import matrix
from numpy import linalg

def strip_non_letters(s:str)->str:
    """Strip all non-letter characters from a string."""
    return ''.join(c for c in s if c.isalpha())

def single(s:str)->dict:
    freq = {}
    for char in s:
        if char.isalpha():
            freq[char] = freq.get(char, 0) + 1
    
    sorted_items_asc = sorted(freq.items(), key=lambda item: item[1], reverse=True)
    return dict(sorted_items_asc)

def digram_(s:str)->dict:
    freq = {}
    for i in range(len(s) - 1):
        if s[i].isalpha() and s[i+1].isalpha():
            pair = s[i:i+2]
            freq[pair] = freq.get(pair, 0) + 1
    sorted_items_asc = sorted(freq.items(), key=lambda item: item[1], reverse=True)
    return dict(sorted_items_asc)

def trigram(s:str)->dict:
    freq = {}
    for i in range(len(s) - 2):
        if s[i].isalpha() and s[i+1].isalpha() and s[i+2].isalpha():
            triplet = s[i:i+3]
            freq[triplet] = freq.get(triplet, 0) + 1
    sorted_items_asc = sorted(freq.items(), key=lambda item: item[1], reverse=True)
    return dict(sorted_items_asc)

def load_file(path: str)->str:
    print(f"Loading file from {path}")
    file = open(path, 'r')
    data = file.read()
    file.close()
    return data

def write_to_file(file_name:str, data:str):
    file = open(file_name, 'w')
    file.write(data)
    file.close()

def append_to_file(file_name:str, data:str):
    file = open(file_name, 'a')
    file.write(data)
    file.close()

def modMatInv(A,p):       # Finds the inverse of matrix A mod p
  n=len(A)
  A=matrix(A)
  adj=numpy.zeros(shape=(n,n))
  for i in range(0,n):
    for j in range(0,n):
      adj[i][j]=((-1)**(i+j)*int(round(linalg.det(minor(A,j,i)))))%p
  return (modInv(int(round(linalg.det(A))),p)*adj)%p

def modInv(a,p):          # Finds the inverse of a mod p, if it exists
  for i in range(1,p):
    if (i*a)%p==1:
      return i
  raise ValueError(str(a)+" has no inverse mod "+str(p))

def minor(A,i,j):    # Return matrix A with the ith row and jth column deleted
  A=numpy.array(A)
  minor=numpy.zeros(shape=(len(A)-1,len(A)-1))
  p=0
  for s in range(0,len(minor)):
    if p==i:
      p=p+1
    q=0
    for t in range(0,len(minor)):
      if q==j:
        q=q+1
      minor[s][t]=A[p][q]
      q=q+1
    p=p+1
  return minor