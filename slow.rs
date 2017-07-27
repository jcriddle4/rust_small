
extern crate byteorder;

use std::mem;
use byteorder::{LittleEndian};
use byteorder::ByteOrder;

type A256 = [u32; 256];


pub struct MyStruct {
    s0: A256
    ,s1: A256
    ,s2: A256
    ,s3: A256
    ,s4: A256
    ,s5: A256
    ,s6: A256
    ,s7: A256
    ,s8: A256
    ,s9: A256
    ,s10: A256
    ,s11: A256
    ,s12: A256
    ,s13: A256
    ,s14: A256
    ,s15: A256
    ,s16: A256
    ,s17: A256
    ,s18: A256
    ,s19: A256
    ,s20: A256
    ,s21: A256
    ,s22: A256
    ,s23: A256
    ,s24: A256
    ,s25: A256
}

const EXPECTED_SIZE: usize = 90000;

fn slow_stuff(){
    let mut mystruct : MyStruct;
    let mut ee: [u8; EXPECTED_SIZE];

    unsafe {
        mystruct = mem::uninitialized();
        ee = mem::uninitialized();
    }

    let mut ptr: usize = 0;
    fn whack_ar(a: &mut [u32], ptr: &mut usize, ee: &mut [u8; EXPECTED_SIZE]) {
        for elem in a.iter_mut() {
            *elem = LittleEndian::read_u32(&mut ee[*ptr..(*ptr+4)]);
            *ptr = *ptr + 4;
        }
    }

    whack_ar(&mut mystruct.s0, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s1, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s2, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s3, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s4, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s5, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s6, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s7, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s8, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s9, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s10, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s11, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s12, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s13, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s14, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s15, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s16, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s17, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s18, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s19, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s20, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s21, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s22, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s23, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s24, &mut ptr,&mut ee);
    whack_ar(&mut mystruct.s25, &mut ptr,&mut ee);

}

fn main() {

    slow_stuff();

    println!("Hello World");

}
