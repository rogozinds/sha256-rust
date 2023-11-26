use std::fs::File;

use crate::sha256::sha256 as my_sha256;

mod constants;
mod sha256;
fn main() {
    let f = File::open("/home/dmitrii/tmp/random_file.bin").unwrap();
    // let f = File::open("/home/dmitrii/tmp/abc_64.txt").unwrap();
    let hash = my_sha256(f);

    println!("{:?}", u32_array_to_hex_string(hash));
}

fn u32_array_to_hex_string(arr: [u32; 8]) -> String {
    arr.iter()
       .fold(String::new(), |acc, &num| acc + &format!("{:08x}", num))
}