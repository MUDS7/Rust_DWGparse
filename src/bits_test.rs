use crate::shapes::Bit_Chain;
use crate::bits::{bit_read_B, bit_wrtite_B, bit_read_BB, bit_read_3B, bit_advance_position, bit_write_3B, bit_read_4BITS, bit_read_RL, bit_read_BLL};


#[test]
fn  bit_advance_position_tests(){
    let  mut bitchain=Bit_Chain::strtobt("10101010");
    if bitchain.bit==0&&bitchain.byte==0{
        println!("bit_advance_position successful");
    }else{
        println!("bit_advance_position failed");
    }
    println!("bitchain={:?}",bitchain.chain);
}
#[test]
fn bit_read_B_tests(){
    let mut bitchain=Bit_Chain::strtobt("101010");
    println!("chain={:?}",bitchain.chain);
    let result=bit_read_B(&mut bitchain);
    if (result==0x01){
        println!("success");
    }else{
        println!("failed {}",result);
    }
    println!("chian={}",bitchain.chain[0]);
}
#[test]
fn bit_write_B(){
    let mut bitchain=Bit_Chain::strtobt("0000000");
    bit_wrtite_B(&mut bitchain,1);
    if bitchain.chain[0]==0x80{
        println!("successful");
    }else{
        println!("failed");
    }
}
#[test]
fn bit_read_BB_test(){
    let mut bitchain=Bit_Chain::strtobt("10101010");
    let result=bit_read_BB(&mut bitchain);
    println!("result={}",result);
}
#[test]
fn bit_read_3B_tests(){
    let mut bitchain=Bit_Chain::strtobt("100111");
    let mut result=bit_read_3B(&mut bitchain);
    println!("chain={:?}",bitchain.chain);
    if result==2{
        println!("1 result={},successful",result);
    }else {
        println!("1 failed result={}",result);
        result=bit_read_3B(&mut bitchain);
    }
    if result==0{
        println!("2 result={},successful",result);
    }else {
        println!("2 failed result={}",result);
        result=bit_read_3B(&mut bitchain);
    }
    if result==7{
        println!("3 result={},successful",result);
    }else {
        println!("3 failed result={}",result);
        result=bit_read_3B(&mut bitchain);
    }
}
#[test]
fn bit_write_3B_tests(){
    let mut loglevle=0;
    let mut bitchain=Bit_Chain::strtobt("01000000");
    bit_advance_position(&mut bitchain,2);
    bit_write_3B(&mut bitchain,0x2);
    if bitchain.chain[0]==80{
        println!("successful");
    }
}
#[test]
fn bit_read_4BITS_tests(){
    let mut bitchain=Bit_Chain::strtobt("11111111");
    let result =bit_read_4BITS(&mut bitchain);
    if result==0xF{
        println!("successful");
    }
}
pub fn test_4bits(binary_str:&str,tep:u8){
    let mut bitchain=Bit_Chain::strtobt(binary_str);
    let result=bit_read_4BITS(&mut bitchain);
    if result==tep{
        println!("binary_str {} successful",binary_str);
    }else{
        dbg!("bit_read_4BITS 0x{} != 0xF dat:{}",result,bitchain.chain[0]);
    }
}
#[test]
fn test_4bits_tests(){
    test_4bits ("0000", 0x0);
    test_4bits ("0001", 0x1);
    test_4bits ("0010", 0x2);
    test_4bits ("0011", 0x3);
    test_4bits ("0100", 0x4);
    test_4bits ("0101", 0x5);
    test_4bits ("0111", 0x7);
    test_4bits ("1000", 0x8);
    test_4bits ("1001", 0x9);
    test_4bits ("1100", 0xC);
    test_4bits ("1101", 0xD);
    test_4bits ("1111", 0xF);
}
#[test]
fn bit_read_BLL_tests(){
    let mut bitchain=Bit_Chain::strtobt("00100000011");
    let result=bit_read_BLL(&mut bitchain);
    println!("result={}",result);
}