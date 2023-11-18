
// //w  w-bit word in our case w-32

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

