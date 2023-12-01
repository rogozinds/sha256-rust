use std::fs::File;
mod constants;

use sha256::sha256 as my_sha256;
use sha256::u32_array_to_hex_string;

fn main() {
    let f = File::open("/home/drogozin/tmp/random_file.bin").unwrap();
    let hash = my_sha256(f);

    println!("{:?}", u32_array_to_hex_string(hash));
}
