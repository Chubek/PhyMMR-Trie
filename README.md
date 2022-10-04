# PhyMMR Trie

1. Start new Python virtualenv.
2. Install Rust and Cargo.
3. `pip3 install maturin`.
4. `maturin develop`
5. `python3 pytest.py`

Note: The script will generate a key between 10 and 1000 and a value of between 1e2 and 1e7. Generating key and value may take time depending on your system. Insertion/Getting is reliant on size of key and value. It runs 1000 iterations but you can change it to howmuchever you want.