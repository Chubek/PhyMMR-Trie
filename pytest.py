from phymmr_trie import PhyTrie
from time import time
from string import ascii_letters
from random import randint
from tqdm import tqdm

def random_str(length):
    return "".join([ascii_letters[randint(0, len(ascii_letters) - 1)] for _ in range(length)])

print("Generating key/value")

key = random_str(randint(42, 49))
value = random_str(randint(300, 400))


print(f"Key length: {len(key)}")
print(f"Value length: {len(value)}")

trie = PhyTrie()

num_loop = 17696208

pbar = tqdm(total=num)

print("\nStarting test...\n")
t1 = time()
for _ in range(num_loop):
    trie.insert(key, value)
    trie.get(key)
    pbar.update(1)

pbar.close()

t2 = time()


print(f"\n{num_loop} inserts and gets done in {t2 - t1} seconds\n")