use crate::shapes::Bit_Chain;
use lazy_static::lazy_static;
use log::{error};
static mut loglevel:i32=0;
pub fn bit_advance_position(dat:&mut Bit_Chain, advance:u32) ->i32{
    let pos=bit_position(dat);
    let endpos=(dat.size*8);
    let bits=dat.bit as u32+advance;
    let mut result=0;
    if (pos+advance)>endpos{
        unsafe {
            loglevel= (dat.opts & 0xf) as i32;
            result=loglevel;
        }
        error!("__FUNCTION__ buffer overflow at pos {}.{}, size {}, advance by %{}",
                dat.byte,dat.bit,dat.size,advance);
    }else if pos+advance<0{
        unsafe {
            loglevel= ((dat.opts & 0xf) as i32);
            result=loglevel;
        }
        error!("buffer underflow at pos {}.{}, size {}, advance by {}",
               dat.byte,dat.bit,dat.size,advance);
        dat.byte=0;
        dat.bit=0;
        return 0;
    }
    dat.byte+=bits>>3;
    dat.bit= (bits & 7) as u8;
    result
}
pub  fn bit_read_B(mut dat:&mut Bit_Chain) ->u8{
    let mut byte=dat.chain[dat.byte as usize];
    let mut result=(byte&(0x80>>dat.bit))>>(7-dat.bit);
    bit_advance_position(&mut dat, 1);
    result
}
pub fn bit_wrtite_B(mut dat:&mut Bit_Chain,value:u8){
    if value!=0{
        dat.chain[dat.byte as usize]|=0x80>>dat.bit;
        println!("bit.chain[0]={}",dat.chain[0]);
    }else{
        dat.chain[dat.byte as usize]&=!(0x80>>dat.bit);
    }
    bit_advance_position(&mut dat,1);
}
pub fn bit_read_BB(dat:&mut Bit_Chain)->u8{
    let mut result=0;
    let mut byte=dat.chain[dat.byte as usize];
    if dat.bit<7{
        result=(byte&(0xc0>>dat.bit))>>(6-dat.bit);
    }else{
        result=(byte&0x01)<<1;
        if dat.byte<dat.size-1{
            byte=dat.chain[(dat.byte+1) as usize];
            result|=(byte&0x80)>>7;
        }
    }
    bit_advance_position(dat,2);
    result
}
pub fn bit_read_3B(dat:&mut Bit_Chain)->u8{
    let result=bit_read_B(dat);
    if result!=0{
        let (mut next)=bit_read_B(dat);
        if next!=0{
            let (mut nexe)=bit_read_B(dat);
            if next!=0{next=7}else{next=0}
            return next;
        }else{
            return 2;
        }
    }else{
        return 0;
    }
}
pub fn bit_write_3B(dat:&mut Bit_Chain, mut value:u8,){
    if value>7{
        unsafe {
             loglevel= (dat.opts  & 0xf) as i32;
        }
        error!("Invalid bit_write_3B value {} > 7", value);
        return ;
    }
    bit_wrtite_B(dat,value&1);
    if value!=0{
        value>>=1;
        bit_wrtite_B(dat,value&1);
        if value!=0{
            value>>=1;
            bit_wrtite_B(dat,value&1);
        }
    }
}
pub fn bit_read_4BITS(dat:&mut Bit_Chain)->u8{
    let b=bit_read_B(dat)<<3|
              bit_read_B(dat)<<2|
              bit_read_B(dat)<<1|
              bit_read_B(dat);
    b
}
pub fn bit_read_BLL(mut dat:&mut Bit_Chain) ->u32{
    let mut result=0;
    let len=(bit_read_BB(&mut dat)<<1)|(bit_read_B(&mut dat));
    println!("len={}",len);
    match len {
        1=>return bit_read_RC(&mut dat),
        2=>return bit_read_RS(&mut dat),
        4=>return bit_read_RL(&mut dat),
        _=>{
            for i in 0..len{
                result<<=8;
                result|=bit_read_RC(&mut dat);
            }
            result as u32
        }
    }
}
pub fn bit_read_RC(dat:&mut Bit_Chain)->u32{
    let mut byte=dat.chain[dat.byte as usize];
    let mut result=0;
    dat.size= dat.chain.len() as u32;
    if dat.bit==0{
        result=byte;
    }else{
        result=byte<<dat.bit;
        if dat.byte==0&&dat.size==0{
            byte=dat.chain[(dat.byte+1) as usize];
            result|=byte>>(8-dat.bit);
        } else if dat.byte <(dat.size-1){
            byte=dat.chain[(dat.byte+1) as usize];
            result|=byte>>(8-dat.bit);
        }else{
            unsafe {
                loglevel= (dat.opts & 0xf) as i32;
            }
            error!("__FUNCTION__ buffer overflow at {}",dat.byte + 1);
            return 0;
        }
    }
    bit_advance_position(dat,8);
    result as u32
}
pub fn bit_read_RS(dat:&mut Bit_Chain)->u32{
    let byte1=bit_read_RC(dat);
    let byte2=bit_read_RC(dat);
    (byte2<<8|byte1)
}
pub fn bit_read_RL(dat:&mut Bit_Chain)->u32{
    let word1=bit_read_RS(dat);
    let word2=bit_read_RS(dat);
    ((word2 as u32)<<16)|(word1 as u32)
}
pub fn bit_position(dat:&mut Bit_Chain)->u32{
    ((dat.byte * 8) + (dat.bit as u32 & 7))
}
pub fn bit_chain_alloc(dat:&mut Bit_Chain){
    bit_chain_alloc_size(dat,2)
}
pub fn bit_chain_alloc_size(dat:&mut Bit_Chain,size:u32){
    if dat.size==0{
        bit_chain_init(dat,size)
    }else{
        let mut arr=Vec::with_capacity((size as usize * dat.chain.len()));
        for data in dat.chain.iter(){
            arr.push(*data);
        }
        for _ in dat.chain.len()..arr.len(){
            arr.push(0);
        }
        dat.chain=arr;
        dat.size+=size;
    }
}
pub fn bit_chain_init(dat:&mut Bit_Chain,size:u32){

    if (dat.chain.len()==0){
        unsafe{
            loglevel= (dat.opts & 0xf) as i32;
        }
        error!("Out of memory");
    }
    dat.size= size;
    dat.byte=0;
    dat.bit=0;
}