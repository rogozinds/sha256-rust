use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use sha256::{sha256, u32_array_to_hex_string};

#[test]
fn test_sha256() {
    let path = Path::new("tests/resources/hashes.txt");
    let file = File::open(&path).expect("Could not open hashes.txt");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let parts: Vec<&str> = line.split(':').collect();
        assert_eq!(parts.len(), 2, "Invalid format in hashes.txt");

        let filename = parts[0];
        let expected_hash = parts[1];
        
        let file_path = Path::new("tests/resources").join(filename);
        let file = File::open(file_path).expect("Could not open test file");
        let computed_hash = u32_array_to_hex_string(sha256(file));


        assert_eq!(expected_hash, computed_hash);
        assert_eq!(expected_hash, computed_hash, "Hash mismatch for file {}", filename);
    }
}