#[test]
fn main() {
    assert!(1u32 + 2 == 3);
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);
    assert!(3 * 50 == 150);
    let tolerance = 0.00001_f64;
    assert!((9.6_f64 / 3.2_f64 - 3.0_f64).abs() < tolerance);
    assert!(24 % 5 == 4);
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
