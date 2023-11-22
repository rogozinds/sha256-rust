use crate::constants::K;
use crate::constants::H;
// //w  w-bit word in our case w-32

//this is doing it for the whole file.
fn encode_all(buf:&mut Vec<u8>) {
 // read a file:
//  let N = 10; // this is the n blocks in the file
//  for i 1 .. N {

//  }
}
fn encode(mes: [u32;16]) -> [u32; 8]{
    let mut hash = H.clone();
    _encode(mes, &mut hash);
    hash
}
//h is coming from previous iteration
fn _encode(mes: [u32;16], hash: &mut [u32; 8]) {
    //Write code to extract mes from message, or just pass it like that.
    let mut a = hash[0];
    let mut b = hash[1];
    let mut c = hash[2];
    let mut d = hash[3];
    let mut e = hash[4];
    let mut f = hash[5];
    let mut g = hash[6];
    let mut h = hash[7];
    // 
    //prepare the the message schedule, have no fucking idea how it's working we will figure out later
    //word shedule is W
    //mes should be somehow read from message
    let mut W:[u32;64] = [0;64];
    for t in 0 ..16 {
            W[t] =  mes[t];
    } 
    for t in 16 ..64 {
            let a = sigma_1_256(W[t-2]);
            let b = sigma_0_256(W[t-15]);
            W[t] =  a.wrapping_add( W[t-7]).wrapping_add(b).wrapping_add(W[t-16]);
    } 
    for t in 0..64 {
        let t1 = h.wrapping_add(epsil_1_256(e))
                .wrapping_add(ch(e, f, g))
                .wrapping_add(K[t])
                .wrapping_add(W[t]);

        let t2 = epsil_0_256(a).wrapping_add(maj(a, b, c));
        h = g;
        g= f;
        f =e;
        e = d.wrapping_add(t1);
        d =c;
        c=b;
        b=a;
        a = t1.wrapping_add(t2);
    }
    hash[0] = hash[0].wrapping_add(a);
    hash[1] = hash[1].wrapping_add(b);
    hash[2] = hash[2].wrapping_add(c);
    hash[3] = hash[3].wrapping_add(d);
    hash[4] = hash[4].wrapping_add(e);
    hash[5] = hash[5].wrapping_add(f);
    hash[6] = hash[6].wrapping_add(g);
    hash[7] = hash[7].wrapping_add(h);
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

pub fn pad_message_buf(buf: &mut [u8], read_size: usize) ->[u32; 16] {
    if buf.len() != 64  || read_size>64{
        panic!("Buffer length should be 64 bytes");
    }
    let  original_len_bits = (read_size as u64) * 8 ;
    let mut index=read_size; //index is read_size-1, but we add 1 byte, according to alg

    let modified_buf_len = ((original_len_bits+1)  % 512) as u64;
    //check if it's zero no need to pad.
    let n_zeros = 448 - modified_buf_len;
    let n_zeros_bytes = n_zeros / 8;
    buf[index] = 0x80;
    index+=1;
    for i in index..index+n_zeros_bytes as usize {
        buf[i]=0;
    }
    index+=n_zeros_bytes as usize; 
    let l_asbytes = u64_to_byte_block(original_len_bits);
    for i in 0..8 {
        buf[index+i]=l_asbytes[i];
    }
        
    // Convert the padded buffer to [u32; 16]
    let mut output = [0u32; 16];
    for (i, chunk) in buf.chunks(4).enumerate() {
        output[i] = u32::from_be_bytes(chunk.try_into().unwrap());
    }

    output

}
pub fn convert_to_u32_array(buf: &Vec<u8>) -> [u32; 16] {
    if buf.len() != 64 {
        panic!("Buffer length must be exactly 64 bytes to convert to [u32; 16]");
    }

    let mut array = [0u32; 16];
    for (i, chunk) in buf.chunks(4).enumerate() {
        let bytes: [u8; 4] = chunk.try_into().expect("Slice with incorrect length");
        array[i] = u32::from_be_bytes(bytes);
    }

    array
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

//TODO fix tests, the problem is that pad_message has a Vector and encode expects an array.
#[test]
fn encode_on_empty_string_test_vector() {
   let mut val :Vec<u8>= vec![]; 
   let mut val = val.clone();
   pad_message(&mut val);
   let mes =convert_to_u32_array(&val);
   let hash = encode(mes);
    let expected = [
        0xe3b0c442,
        0x98fc1c14,
        0x9afbf4c8,
        0x996fb924,
        0x27ae41e4,
        0x649b934c,
        0xa495991b,
        0x7852b855
    ];
    assert_eq!(expected , hash); 

}
#[test]
fn pad_vec_and_pad_buf_are_same() {

   let mut val :Vec<u8>= vec![97,98,99]; 
   pad_message(&mut val);
   let mes_vec =convert_to_u32_array(&val);
   let mut buf :[u8;64]= [0;64];
   buf[0]=97;
   buf[1]=98;
   buf[2]=99;
   let mes_buf = pad_message_buf(&mut buf, 3);

    assert_eq!(mes_vec , mes_buf); 
}
#[test]
fn encode_on_abc_string_test_vector() {
   let mut val :Vec<u8>= vec![97,98,99]; 
   let mut val = val.clone();
   pad_message(&mut val);
   let mes =convert_to_u32_array(&val);
   let hash = encode(mes);
    let expected = [
        0xba7816bf,
        0x8f01cfea,
        0x414140de,
        0x5dae2223,
        0xb00361a3,
        0x96177a9c,
        0xb410ff61,
        0xf20015ad

    ];
    assert_eq!(expected , hash); 

}
#[test]
fn encode_on_rc4_stream_test_vector() {
   let mut val :Vec<u8>= vec![
    ]; 

   let str= "de188941a3375d3a8a061e67576e926d";
   for i in 0..16 {
        if let parsed = u8::from_str_radix(&str[i*2..i*2+2],16) {
            val.push(parsed.unwrap());
        }
   }
   pad_message(&mut val);
   let mes =convert_to_u32_array(&val);
   let hash = encode(mes);
    let expected = [
        0x067c5312,
        0x69735ca7,
        0xf541fdac,
        0xa8f0dc76,
        0x305d3cad,
        0xa140f893,
        0x72a410fe,
        0x5eff6e4d
    ];
    assert_eq!(expected , hash); 

}
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

