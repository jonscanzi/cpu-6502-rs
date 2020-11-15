#![allow(dead_code)]

mod cpu;
mod assembler;

fn main() {
    let _test = 0b10000000u8;
    let _testt = 0b01000000u8;
    let _testtt = 0b00100000u8;

    // println!("{}", _testt.overflowing_shl(2).1);
    assembler::parse_program("dfsfsdf");
}