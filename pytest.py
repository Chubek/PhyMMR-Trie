from phymmr_trie import PhyTrie
from time import time
from string import ascii_letters
from random import randint
from tqdm import tqdm

def random_str(length):
    return "".join([ascii_letters[randint(0, len(ascii_letters) - 1)] for _ in range(length)])


key = random_str(randint(10, 1000))
value = random_str(randint(1e2, 1e7))

print(f"Key length: {len(key)}")
print(f"Value length: {len(value)}")

trie = PhyTrie()

pbar = tqdm(total=1000)

print("\nStarting test...\n")
t1 = time()
for _ in range(1000):
    trie.insert(key, value)
    trie.get(key)
    pbar.update(1)

pbar.close()

t2 = time()


print(f"\n1000 inserts and gets done in {t2 - t1} seconds\n")