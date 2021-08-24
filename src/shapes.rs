use crate::tests_common::{bitprepare, bit_write_B};
use nom::AsChar;

#[derive(Debug)]
pub enum Dwg_Version_Type{
    R_INVALID,
    R_1_1,	/* MC0.0  MicroCAD Release 1.1 */
    R_1_2,	/* AC1.2  AutoCAD Release 1.2 */
    R_1_3,	/* AC1.3  AutoCAD Release 1.3 */
    R_1_4,	/* AC1.40 AutoCAD Release 1.4 */
    R_1_402b,	/* AC402b AutoCAD Release 1.402b */
    R_2_0,	/* AC1.50 AutoCAD Release 2.0 */
    R_2_1,	/* AC2.10 AutoCAD Release 2.10 */
    R_2_21,	/* AC2.21 AutoCAD Release 2.21 */
    R_2_22,	/* AC2.22 AutoCAD Release 2.22 */
    R_2_4,	/* AC1001 AutoCAD Release 2.4 */
    R_2_5,	/* AC1002 AutoCAD Release 2.5 */
    R_2_6,	/* AC1003 AutoCAD Release 2.6 */
    R_9,		/* AC1004 AutoCAD Release 9 */
    R_9c1,	/* AC1005 AutoCAD Release 9c1 */
    R_10,		/* AC1006 AutoCAD Release 10 */
    R_10c1,	/* AC1007 AutoCAD Release 10c1 */
    R_10c2,	/* AC1008 AutoCAD Release 10c2 */
    R_11,		/* AC1009 AutoCAD Release 11/12 (LT R1/R2) */
    R_12,		/* AC1010 AutoCAD Release 12 */
    R_12c1,	/* AC1011 AutoCAD Release 12c1 */
    R_13,		/* AC1012 AutoCAD Release 13 */
    R_13c3,	/* AC1013 AutoCAD Release 13C3 */
    R_14,		/* AC1014 AutoCAD Release 14 */
    R_2000,	/* AC1015 AutoCAD Release 2000 */
    R_2004,	/* AC1018 AutoCAD Release 2004 (includes versions AC1019/0x19 and AC1020/0x1a) */
    R_2007,	/* AC1021 AutoCAD Release 2007 - 2019*/
    R_2010,	/* AC1024 AutoCAD Release 2010 - 2012 */
    R_2013,	/* AC1027 AutoCAD Release 2013 - 2017 */
    R_2018,	/* AC1032 AutoCAD Release 2018 - 2021 */
    R_AFTER,
    Null
}
#[derive(Debug)]
pub struct File;
#[derive(Debug)]
pub struct Bit_Chain{
    pub chain:Vec<u8>,
    pub size:u32,
    pub byte:u32,
    pub bit:u8,
    pub opts:u8,
    pub version:Dwg_Version_Type,
    pub from_version:Dwg_Version_Type,
    pub fh:File,
}
impl Bit_Chain{
    pub fn strtobt(binarystring:&str)->Bit_Chain{
        let binaryarry=binarystring.as_bytes();
        let length=binarystring.len();
        let mut size_needed=length/8;
        if length%8!=0 {
            size_needed+=1;
        }
        let mut dat=bitprepare(size_needed);
        for i in 0..length{
            if binaryarry[i]==48{
                bit_write_B(&mut dat,0);
            }else {
                bit_write_B(&mut dat,1);
            }
        }
        dat.bit=0;
        dat.byte=0;
        dat
    }
}