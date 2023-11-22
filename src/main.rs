use crate::sha256::pad_message;

mod constants;
mod sha256;
fn main() {
   let mut message = Vec::from("abc".as_bytes()); // Convert "abc" to a byte vector
    // pad_message(&mut message);
    let a = 3287351444u32; 
    let b = 1711282176u32;
    let d = 2117632u32;
    let c = a.wrapping_add(b).wrapping_add(d);
    println!("c={}",c);
    // Print the padded message as bytes
    println!("{:?}", message);
}
