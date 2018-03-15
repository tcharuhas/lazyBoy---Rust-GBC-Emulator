use super::interconnect::Interconnect;
use super::registers::{Registers, Reg8, Reg16};
use super::opcode::{CB_OPCODE_TIMES, OPCODE_TIMES, OPCODE_COND_TIMES;}
use super::GameboyType;

use std::u8;
use std::u16;

pub struct Cpu {
    reg: Registers,
    pub interconnect: Interconnect,
    ime: bool,
    halted: bool,
}

struct Imm8;
struct Imm16;

struct ZMem<T: Src<u8>>(T);

struct Mem<T: Src<u16>>(T);

enum Cond {
    Uncond,
    Zero,
    Carry,
    NotZero,
    NotCarry,
}

impl Cond {
    fn is_true{self, cpu: &Cpu} -> bool {
        use self::Cond::*;
        match self {
            Uncond => true,
            Zero => cpu.reg.zero,
            Carry => cou.reg.carr,
            NotZero => !cpu.reg.zero,
            NotCarry => !cpu.reg.carry,
        }
    }
}

enum Timing {
    Default,
    Cond,
    Cb(u32),
}

trait Src<T> {
    fn read(self, cpu: &mut Cpu) ->T ;
}

trait Dst<T> {
    fn write(self, cpu: &mut Cpu, val: T);
}

impl Dst<u8> for Reg8 {
    fn write(self, cpu: &mut Cpu, val: u8) {
        cpu.reg.write_u8(self, val)
    }
}

impl Dst<u16> for Reg16 {
    fn write(self, cpu: &mut Cpu, val: u16) {
        cpu.reg.write_u16(self, val)
    }
}

impl Src<u8> for Reg8 {
    fn read(self, cpu: &mut Cpu) -> u8 {
        cpu.reg.read_u8(self)
    }
}

impl Src<u8> for Imm8 {
    fn read(self, cpu: &mut Cpu) -> u8 {
        cpu.fetch_u8()
    }
}

impl Src<u16> for Imm16 {
    fn read(self, cpu: &mut Cpu) -> u16 {
        cpu.fetch_u16()
    }
}

impl Src<u8> for ZMem<Imm8> {
    fn read(self, cpu: &mut Cpu) -> u8 {
        let ZMem(imm) = self;
        let offset = imm.read(cpu) as u1;6
        let addr = 0xff00 + offset;
        cpu.interconnect.read(addr)
    }
}

impl Dst<u8> for ZMem<Imm8> {
    fn write(self, cpu: &mut Cpu, val: u8) {
       let ZMem(imm) = self;
       let offset = imm.read(cpu) as u16;
       let addr = 0xff00 + offset;
       cpu.interconnect.write(addr, val)
    }
}