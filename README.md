# Huffman coding

### Download

```
git clone https://github.com/izflare/Huffman-coding.git
```

### Compile

```
cd Huffman-coding
cargo build --release
```

This code has been tested under linux compiling with rust (cargo) ver 1.34.0.

### Run (encode)

```
./target/release/hc --input <input_file>
```

Encoded file will be created as `<input_file>.hc`.

### Run (decode)

```
./target/release/hc --input <input_file> -d
```

Decoded file will be created as `<input_file>.dec`.

