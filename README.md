# Huffman coding

### Download

To clone the repository, call

```
git clone https://github.com/izflare/Huffman-coding.git
```

### Compile

This code has been tested under linux compiling with rust (cargo) ver 1.34.0  
After download the repository, 

```
cd Huffman-coding
cargo build --release
```

### Run

After compiling,

```
cd target/release
./hc [-d] --input <input>
```

then the tool run.  
`<input>` is your input text data file.
Set `-r` flag if you want to decode the file.

