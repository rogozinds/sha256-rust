use crate::constants::K;
// //w  w-bit word in our case w-32

//this is doing it for the whole file.
fn encode_all(buf:&mut Vec<u8>) {
 // read a file:
 let N = 10; // this is the n blocks in the file
 for i 1 .. N {

 }
}
//h is coming from previous iteration
fn encode(mes: [u32;64], H:&mut [u32; 8]) {
    //Write code to extract mes from message, or just pass it like that.
    let mut a = H[0];
    let mut b = H[1];
    let mut c = H[2];
    let mut d = H[3];
    let mut e = H[4];
    let mut f = H[5];
    let mut g = H[6];
    let mut h = H[7];
    // 
    //prepare the the message schedule, have no fucking idea how it's working we will figure out later
    //word shedule is W
    //mes should be somehow read from message
    let mut mes: [u32;64] = [0;64];
    let mut W:[u32;64] = [0;64];
    for t in 0 ..16 {
            W[t] =  mes[t];
    } 
    for t in 16 ..64 {
            W[t] =  sigma_1_256(W[t-2]) + W[t-7]  +sigma_0_256(W[t-15]) + W[t-16];
    } 
    for t in 0..64 {
        if t < 16 {
        } else  {

        }
        let t1 = h + epsil_1_256(e) + ch(e, f, g) + K[t]  +W[t];
        let t2 = epsil_0_256(a) + maj(a, b, c);
        h = g;
        g= f;
        f =e;
        e = d + t1;
        d =c;
        c=b;
        b=a;
        a = t1 + t2;
    }
    H[0] = a +H[0];
    H[1] = b +H[1];
    H[2] = c +H[2];
    H[3] = d +H[3];
    H[4] = e +H[4];
    H[5] = f +H[5];   
    H[6] = g +H[6];   
    H[7] = h +H[7];   

}
pub fn pad_message(buf:&mut Vec<u8>) {
    let  original_buf_len_bits = (buf.len() as u64) * 8 ;
    let modified_buf_len = ((original_buf_len_bits+1)  % 512) as u64;
    //check if it's zero no need to pad.
    let n_zeros = 448 - modified_buf_len;
    let n_zeros_bytes = n_zeros / 8;
    buf.push(0x80);
    buf.extend(vec![0u8; n_zeros_bytes as usize]);
    
    let l_asbytes = u64_to_byte_block(original_buf_len_bits);
    buf.extend_from_slice(&l_asbytes);

}
fn u64_to_byte_block(value: u64) -> [u8; 8] {
    let mut byte_block = [0u8; 8];

    for i in 0..8 {
        //Example how it works on 1027 value that is 1024+3
        //Here we go 'pick' only bits that fit into this exact block.
        //For example, a small value like 3, which should result in b0000_0011
        //so value >>0 will be same value then & b1111_1111, we pick only lowest byte 
        // (&FF, ensures we don't get higher bits in the result like, values from a mln.)

        //Let's say the value was not 3, but 1024 +3, 3 would go into to 0th block. Then 1024 will be extracted
        //for i=1 so it will be b0000_0100 for i1.
        //So the result for 1027 val will be byte_block[1] = b0000_0100 byte_block[0]=b0000_0011
        byte_block[7-i] = ((value >> (i * 8)) & 0xFF) as u8;
    }

    byte_block
}
pub fn ch(x:u32, y:u32, z:u32)->u32{
    (x&y) ^ (!x&z)
}

pub fn maj(x:u32, y:u32, z:u32)->u32{
    (x&y) ^ (x&z) ^ (y &z)
}

fn rotr(x:u32, n:u32)->u32{
    _rotr(x,n,32)
}
fn rotl(x:u32, n:u32)->u32{
    _rotl(x,n,32)
}
//Internal
fn _rotr(x:u32, n:u32, num_bits:u32)->u32{
    let left = x>>n;
    let right =  x << num_bits-n;
    left | right
}
fn _rotl(x:u32, n:u32, num_bits:u32)->u32{
    (x<<n) | (x >> num_bits-n)
}
fn shr(x:u32,n:u32)->u32 {
    if n<0 || n> 32 {
        panic!("Error in SHR, n is in wrong range. n={}",n);
    }
    x >> n
}

fn epsil_0_256(x:u32)->u32{
    rotr(x,2) ^ rotr(x,13) ^ rotr(x,22)
}

fn epsil_1_256(x:u32)->u32{
    rotr(x,6) ^ rotr(x,11) ^ rotr(x,25)
}

fn sigma_0_256(x:u32)->u32{
    rotr(x,7) ^ rotr(x,18) ^ shr(x,3)
}

fn sigma_1_256(x:u32)->u32{
    rotr(x,17) ^ rotr(x,19) ^ shr(x,10)
}
#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn check_pad_message_returns_correct_chars() {
    let mut val= vec![97 as u8, 98 as u8, 99 as u8]; //abc
    pad_message(&mut val);
    assert_eq!( 64, val.len()); 

    assert_eq!( 97, val[0]);  //  a b0110_0001
    assert_eq!( 98, val[1]); // b b0110_0010
    assert_eq!( 99, val[2]); //c
    assert_eq!(0x80, val[3] ); //divide bit b1000_0000
    assert_eq!( 24, val[63]);  //len as big Endian
}
#[test]
fn check_pad_message_returns_correct_num() {
    let mut val= vec![3 as u8, 4 as u8];
    pad_message(&mut val);
    assert_eq!( 64, val.len()); 
    //TODO check content
}
#[test]
fn check_pad_message_returns_block_len512() {
    let mut val= vec![3 as u8];
    pad_message(&mut val);
    assert_eq!( 64, val.len()); 
}
#[test]
fn u64_to_byte_block_small_number(){
    let val :u64= 24;
    let actual = u64_to_byte_block(val);
    let mut expected = [0u8; 8];
    expected[7]=0b0001_1000;
    assert_eq!( expected, actual); 
}
#[test]
fn u64_to_byte_block_big_number(){
    let val :u64= 1124;
    let actual = u64_to_byte_block(val);
    let mut expected = [0u8; 8];
    expected[7]=0b0110_0100;
    expected[6]=0b0000_0100;

    assert_eq!( expected, actual); 
}
#[test]
fn rotate_right_on_u32_returns_correct() {
    let num:u32 = 0b00001000;
    let rotated = rotr(num, 2);
    println!("Rotated value {}", rotated);
    assert_eq!(0b0010, rotated); 
}

#[test]
fn rotate_left_on_u32_returns_correct() {
    let num:u32 = 0b00001000;
    let rotated = rotl(num, 2);
    println!("Rotated value {}", rotated);
    assert_eq!(0b100000, rotated); 
}
#[test]
fn ch_on_u32_returns_correct() {
    let x =0b1001_0010;
    let y =0b1111_1010;
    let z =0b1100_0110;

    let res = ch(x, y,z);
    let expected = 0b1101_0110;
    assert_eq!(expected, res); 
}
#[test]
fn maj_on_u32_returns_correct() {
    let x =0b1001_0010;
    let y =0b1111_1010;
    let z =0b1100_0110;

    let res = maj(x, y,z);
    let expected = 0b1101_0010;
    assert_eq!(expected, res); 
}
}

