# Rust SHA256 Implementation

## Overview

This project provides a 0-dependency SHA256 hash implementation in Rust. Designed for efficiency and simplicity, it processes large files in manageable chunks to optimize I/O operations and memory usage.

### Key Features:
- **Efficient File Handling**: By reading parts of the file into a `large_buffer` of 50KB, the implementation significantly reduces I/O operations for large files.
- **Chunk Processing**: The file data is processed in 512-bit chunks, ensuring consistent memory usage and performance across different file sizes.
- **Padding for Last Chunk**: Implements appropriate padding for the last chunk of the file to adhere to SHA256 specifications, ensuring accurate hash computation.
- **No External Dependencies**: Pure Rust implementation without relying on external crates, enhancing portability and ease of integration.

### Usage Example
Please refer to the `main` function in the source code for a usage example.

## Testing
- **Unit Tests**: The project includes unit tests to verify the correctness of the hashing algorithm.
- **Integration Tests**: Integration tests are provided, utilizing randomly generated files to ensure the robustness of the implementation in real-world scenarios.

To execute the tests, run:

```bash
cargo test
```

## Overview
0-dependency sha256 implementation in Rust.
We read part of the file inside the "large_buffer" = 50kb, to avoid million of I/O operations for big files.
Then we process this "large_buffer" in 512 chunks calculating the hash.
For the last chunk of the file we do padding and calculate the final hash.
Check main for usage example.

## Results:
For 1.5 Gb file.
Native sha256 command runs 4.2 sec.
This implemenations ~ 5 sec.

## Create random file in Linux
```bash
dd if=/dev/urandom of=my_random_file.bin bs=1M count=10
```

