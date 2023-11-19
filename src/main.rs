use crate::sha256::pad_message;

mod constants;
mod sha256;
fn main() {
   let mut message = Vec::from("abc".as_bytes()); // Convert "abc" to a byte vector
    pad_message(&mut message);
    
    // Print the padded message as bytes
    println!("{:?}", message);
}
