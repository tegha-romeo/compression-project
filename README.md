# compression-project

# Compression Cli  implented in Rust and Javascript (RlE and LZ77 algorithms)

- A cli tool that compresses and decompreses fils **Run-Length Encoding (RLE)** and **Simplified LZ77**, written in both **Rust** and **JavaScript**.
- supports:

1  compression and decompression

2  rle and lz77 algorithms

3  cli interface

4   dockerised builds

5   Benchmarks  and unit Tests

6  GitHub Actions

**Usage:**

> cd rust-compressor && cargo build --release

## A. rust -version

a) run test:  cargo test

b) Build Docker images: docker build -t rust-compressor .

c) run in docker: 

echo "AAABBBCCC" > input.txt


## Compress
> docker run -v .:/data rust-compressor compress /data/input.txt /data/output.rle --rle      

## Decompress
> docker run -v .:/data rust-compressor decompress /data/output.rle /data/recovered.txt --rle  

## Verify
diff input.txt recovered.txt

## B. Js Version

 a) install and test:  cd js-compressor  && npm install && npx mocha test/*.test.js

b) build docker image: docker build -t js-compressor .

c) run in docker 
echo "AAABBBCCC" > input.txt

## Compress
> docker run -v .:/data js-compressor compress /data/input.txt /data/output.rle --rle      

## Decompress
> docker run -v .:/data js-compressor decompress /data/output.rle /data/recovered.txt --rle  

## Verify
diff input.txt recovered.txt

d) Benchmarking: chmod +x benchmark.s && ./benchmark.sh


