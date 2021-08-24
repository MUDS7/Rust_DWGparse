use crate::shapes::{Bit_Chain, Dwg_Version_Type, File};
use std::alloc::Layout;
use nom::combinator::not;
use nom::AsChar;
use crate::bits::{bit_advance_position, bit_chain_alloc};
use log::error;
pub fn bitprepare(size:usize)->Bit_Chain{
    Bit_Chain{
        chain: vec![0],
        size: (size + 1) as u32,
        byte: 0,
        bit: 0,
        opts: 1,
        version: Dwg_Version_Type::R_2000,
        from_version: Dwg_Version_Type::Null,
        fh: File
    }
}

pub fn bit_write_B(mut dat: &mut Bit_Chain, value:u8){
    if dat.byte>=dat.size{
        println!("dat.size={}",dat.size);
        bit_chain_alloc(dat);
    }
    if value!=0{
        dat.chain[dat.byte as usize]|=0x80>>dat.bit;
    }else{
        dat.chain[dat.byte as usize]&=!(0x80>>dat.bit);
    }
    bit_advance_position(&mut dat,1);
}

