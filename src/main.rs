// type word = u8;
// type doubleword = u16;

use std::ops::Add;
use std::ops::Sub;

/// Reperesents a word in 6502 (i.e. a single byte)
/// Currently stored in native endianness
#[derive(Clone, Copy)]
struct word {
    value: u8,
}

impl word {
    fn from_6502_bytes(data: u8) -> Self {
        Self {
            value: data,
        }
    }

    #[inline]
    fn native_value(&self) -> u8 {
        self.value
    }

    #[inline]
    fn native_value_signed(&self) -> i8 {
        self.value as i8
    }

    #[inline]
    fn cpu_value(&self) -> u8 {
        u8::to_le(self.value)
    }

    #[inline]
    fn cpu_value_signed(&self) -> i8 {
        i8::to_le(self.value as i8)
    }

    fn zero() -> Self {
        Self {
            value: 0,
        }
    }

    fn as_doubleword(&self) -> doubleword {
        doubleword {
            value: self.value as u16
        }
    }
}

impl Add<Self> for word {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value, // No need to worry about endianness for a single byte
        }
    }
}

impl Add<u8> for word {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        Self {
            value: self.value + other, // No need to worry about endianness for a single byte
        }
    }
}

impl Sub<u8> for word {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        Self {
            value: self.value - other, // No need to worry about endianness for a single byte
        }
    }
}

/// Represents a doubleword in 6502 (used for addressing and the PC)
/// Currently stored in native endianness
#[derive(Clone, Copy)]
struct doubleword {
    value: u16,
}

impl doubleword {
    fn from_6502_bytes(data: u16) -> Self {
        Self {
            value: u16::from_le(data),
        }
    }

    #[inline]
    fn native_value(&self) -> u16 {
        self.value
    }

    #[inline]
    fn cpu_value(&self) -> u16 {
        u16::to_le(self.value)
    }

    #[inline]
    fn as_addr(&self) -> usize {
        self.value as usize
    }

    fn from_words(hi: word, lo: word) -> Self {
        Self {
            value: u16::from_le_bytes([lo.native_value(), hi.native_value()]),
        }
    }

    fn to_words(&self) -> [word; 2] {
        let ret = u16::to_le_bytes(self.value);
        [word{value: ret[0]}, word{value: ret[1]}]
    }

    fn zero() -> Self {
        Self {
            value: 0,
        }
    }
}

impl Add<Self> for doubleword {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value, 
        }
    }
}

impl Add<u16> for doubleword {
    type Output = Self;
    fn add(self, other: u16) -> Self {
        Self {
            value: self.value + other, 
        }
    }
}

impl Add<word> for doubleword {
    type Output = Self;
    fn add(self, other: word) -> Self {
        Self {
            value: self.value + (other.value as u16), 
        }
    }
}

impl Add<u8> for doubleword {
    type Output = Self;
    fn add(self, other: u8) -> Self {
        Self {
            value: self.value + (other as u16), 
        }
    }
}

impl Add<i16> for doubleword {
    type Output = Self;
    fn add(self, other: i16) -> Self {
        Self {
            value: (self.value as i16 + other) as u16, 
        }
    }
}


const RAM_SIZE_BYTES: usize = 1024;

struct ram {
    data: [word; RAM_SIZE_BYTES],
}

impl ram {

    fn write(&mut self, address: doubleword, data: word) {
        let address: u16 = address.native_value();
        debug_assert!(address <= 0x07ff);
        self.data[address as usize] = data;
    }

    fn read(&self, address: doubleword) -> word {
        debug_assert!(address.native_value() <= 0x07ff);
        self.data[address.as_addr()]
    }
}

struct cartridge {
    program: [word; 1024*1024],
}

enum MemoryAccessType {
    Store,
    Load,
}

struct memory {
    internal_ram: ram,
    cart: cartridge,


}

impl memory {
    pub fn store(&self, address: doubleword, data: word) {
        let test_val = self.access(address, MemoryAccessType::Store, Some(data));
        debug_assert!(test_val.is_none());
    }

    pub fn load(&self, address: doubleword) -> word {
        self.access(address, MemoryAccessType::Load, None).unwrap()
    }

    fn access(&self, address: doubleword, tpe: MemoryAccessType, data: Option<word>) -> Option<word> {
        match address.native_value() {
            0x0000..=0x1FFF => (), // RAM (repeated)
            0x2000..=0x3FFF => (), // PPU (repeated)
            0x4000..=0x4017 => (), // APU and IO
            0x4018..=0x401F => (), // test Mode
            0x4020..=0xFFFF => (), // cartridge
        }
        None
    }
}


struct system {
    a: word,
    x: word,
    y: word,
    pc: doubleword,
    s: word,
    p: word,


    mem: memory,
}


const C_BIT: u8 = (1 << 0);
const Z_BIT: u8 = (1 << 1);
const I_BIT: u8 = (1 << 2);
const D_BIT: u8 = (1 << 3);
const B_BIT: u8 = (1 << 4);
const V_BIT: u8 = (1 << 6);
const N_BIT: u8 = (1 << 7);

impl system {

    #[inline]
    fn reset(&mut self) {
        self.a = word::zero();
        self.x = word::zero();
        self.y = word::zero();
        self.s = word::zero();
        self.p = word::zero();
        self.pc = doubleword::zero();
    }

    #[inline]
    fn low_nibble(byte: word) -> u8 {
        //no need to worry about endianness since it's a single byte
        byte.native_value()
    }
    
    #[inline]
    fn high_nibble(byte: word) -> u8 {
        (byte.native_value() & 0xf0) >> 4
    }

    #[inline]
    fn illegal_op(instr: word) {
        panic!("Error: illegal opcode {}", instr.native_value()); // TODO: while not wrong since a single byte, should not use native_value but 6502_value
    }

    #[inline]
    fn store(&self, address: doubleword, data: word) {
        self.mem.store(address, data);
    }

    #[inline]
    fn load(&self, address: doubleword) -> word {
        self.mem.load(address)
    }

    /// Load from memory, intepreting the value as a signed 16-bit address offset
    fn load_offset(&self, address: doubleword) -> i16 {
        self.mem.load(address).native_value() as i16

    }

    #[inline]
    fn load_doubleword(&self, address: doubleword) -> doubleword {
        let lo = self.mem.load(address);
        let hi = self.mem.load(address + 1u8);

        doubleword::from_words(hi, lo)
    }


    #[inline]
    fn advance_exec(&mut self) {
        let next_instr = self.mem.load(self.pc);
        self.exec(next_instr)
    }


    #[inline]
    fn push_word(&mut self, data: word) {
        self.mem.store(self.s.as_doubleword(), data);
        self.s = self.s + 1; // TODO: might need to add a modulo / wraparound for stack? Check 6502 docs
    }

    #[inline]
    fn push_doubleword(&mut self, data: doubleword) {
        let words = data.to_words();
        self.mem.store(self.s.as_doubleword(), words[0]);
        self.mem.store(self.s.as_doubleword() + 1u8, words[1]);
        self.s = self.s + 2;
    }

    /// More comonly called 'pop
    #[inline]
    fn pull_word(&mut self) -> word {
        self.s = self.s - 1u8;
        self.load(self.s.as_doubleword())

    }

    /// More comonly called 'pop
    #[inline]
    fn pull_doubleword(&mut self) -> doubleword {
        self.s = self.s - 2u8;
        self.load_doubleword(self.s.as_doubleword())
    }

    // These 3 advance functions might seem a bit overkill,
    // but they are in case advancing ends up being more
    // complicated that this

    #[inline]
    /// Advance program execution by 1 byte
    fn advance_pc_1(&mut self) {
        self.pc = self.pc + 1u8;
    }

    #[inline]
    /// Advance program execution by 2 bytes
    fn advance_pc_2(&mut self) {
        self.pc = self.pc + 2u8;
    }

    #[inline]
    /// Advance program execution by 3 bytes
    fn advance_pc_3(&mut self) {
        self.pc = self.pc + 3u8;
    }

    #[inline]
    fn relative_jump(&mut self, offset: word) {
        self.pc = self.pc + offset; // TODO: check that signed vs unsigned is not a problem (shouldn't be with 2's complement)
    }

    #[inline]
    fn C(&self) -> bool {
        (self.p.native_value() & C_BIT) == C_BIT
    }

    #[inline]
    fn set_C(&mut self) {
        self.p = word {value: self.p.native_value() | C_BIT};
    }

    #[inline]
    fn clear_C(&mut self) {
        self.p = word {value: self.p.native_value() & !C_BIT};
    }

    #[inline]
    fn Z(&self) -> bool {
        (self.p.native_value() & Z_BIT) == Z_BIT
    }

    #[inline]
    fn set_Z(&mut self) {
        self.p = word {value: self.p.native_value() | Z_BIT};
    }

    #[inline]
    fn clear_Z(&mut self) {
        self.p = word {value: self.p.native_value() & !Z_BIT};
    }

    #[inline]
    fn I(&self) -> bool {
        (self.p.native_value() & I_BIT) == I_BIT
    }

    #[inline]
    fn set_I(&mut self) {
        self.p = word {value: self.p.native_value() | I_BIT};
    }

    #[inline]
    fn clear_I(&mut self) {
        self.p = word {value: self.p.native_value() & !I_BIT};
    }

    #[inline]
    fn D(&self) -> bool {
        (self.p.native_value() & D_BIT) == D_BIT
    }

    #[inline]
    fn set_D(&mut self) {
        self.p = word {value: self.p.native_value() | D_BIT};
    }

    #[inline]
    fn clear_D(&mut self) {
        self.p = word {value: self.p.native_value() & !D_BIT};
    }

    #[inline]
    fn B(&self) -> bool {
        (self.p.native_value() & B_BIT) == B_BIT
    }

    #[inline]
    fn set_B(&mut self) {
        self.p = word {value: self.p.native_value() | B_BIT};
    }

    #[inline]
    fn clear_B(&mut self) {
        self.p = word {value: self.p.native_value() & !B_BIT};
    }

    #[inline]
    fn V(&self) -> bool {
        (self.p.native_value() & V_BIT) == V_BIT
    }

    #[inline]
    fn set_V(&mut self) {
        self.p = word {value: self.p.native_value() | V_BIT};
    }

    #[inline]
    fn clear_V(&mut self) {
        self.p = word {value: self.p.native_value() & !V_BIT};
    }

    #[inline]
    fn N(&self) -> bool {
        (self.p.native_value() & N_BIT) == N_BIT
    }

    #[inline]
    fn set_N(&mut self) {
        self.p = word {value: self.p.native_value() | N_BIT};
    }

    #[inline]
    fn clear_N(&mut self) {
        self.p = word {value: self.p.native_value() & !N_BIT};
    }
    




    fn exec(&mut self, instr: word) {
        // Matrix evaluation inspired by https://www.masswerk.at/6502/6502_instruction_set.html
        match Self::low_nibble(instr) {
            0x0 => {
                match Self::high_nibble(instr) {
                    0x0 => { // BRK
                        unimplemented!();
                    },
                    0x1 => { // BPL
                        unimplemented!();
                    },
                    0x2 => { // JSR
                        self.push_doubleword(self.pc);
                        let lo = self.mem.load(self.pc + 1u8);
                        let hi = self.mem.load(self.pc + 2u8);
                        self.pc = doubleword::from_words(hi, lo);
                    },
                    0x3 => { // BMI
                        if self.N() {
                            self.pc = self.pc + self.load_offset(self.pc + 1u8)
                        }
                        else {
                            self.pc = self.pc + 2u8;
                        }
                    },
                    0x4 => { // RTI
                        unimplemented!();
                    },
                    0x5 => { // BVC
                        if !self.V() {
                            self.pc = self.pc + self.load_offset(self.pc + 1u8); //TODO: might want to make of this pattern a function or macro
                        }
                        else {
                            self.pc = self.pc + 2u8;
                        }
                    },
                    0x6 => { // RTS
                        self.pc = self.pull_doubleword();
                    },
                    0x7 => { // BVS
                        if self.V() {
                            self.pc = self.pc + self.load_offset(self.pc + 1u8);
                        }
                        else {

                        }
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // BCC
                        unimplemented!();
                    },
                    0xA => { // LDY
                        unimplemented!();
                    },
                    0xB => { // BCS
                        unimplemented!();
                    },
                    0xC => { // CPY
                        unimplemented!();
                    },
                    0xD => { // BNE
                        unimplemented!();
                    },
                    0xE => { // CPX
                        unimplemented!();
                    },
                    0xF => { // BEQ
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x1 => { // ind
                match Self::high_nibble(instr) {
                    0x0 => { // ORA X
                        unimplemented!();
                    },
                    0x1 => { // ORA ind
                        unimplemented!();
                    },
                    0x2 => { // AND X
                        unimplemented!();
                    },
                    0x3 => { // AND ind
                        unimplemented!();
                    },
                    0x4 => { // EOR X
                        unimplemented!();
                    },
                    0x5 => { // EOR ind
                        unimplemented!();
                    },
                    0x6 => { // ADC X
                        unimplemented!();
                    },
                    0x7 => { // ADC ind
                        unimplemented!();
                    },
                    0x8 => { // STA X
                        unimplemented!();
                    },
                    0x9 => { // STA ind
                        unimplemented!();
                    },
                    0xA => { // LDA X
                        unimplemented!();
                    },
                    0xB => { // LDA ind
                        unimplemented!();
                    },
                    0xC => { // CMP X
                        unimplemented!();
                    },
                    0xD => { // CMP ind
                        unimplemented!();
                    },
                    0xE => { // SBC X
                        unimplemented!();
                    },
                    0xF => { // SBC ind
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x2 => {
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // Illegal
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // Illegal
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // Illegal
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // LDX
                        unimplemented!();
                    },
                    0xB => { // Illegal
                        unimplemented!();
                    },
                    0xC => { // Illegal
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // Illegal
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x3 => {
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // Illegal
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // Illegal
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // Illegal
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // Illegal
                        unimplemented!();
                    },
                    0xB => { // Illegal
                        unimplemented!();
                    },
                    0xC => { // Illegal
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // Illegal
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x4 => { // zpg
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // BIT
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // Illegal
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // Illegal
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // STY
                        unimplemented!();
                    },
                    0x9 => { // STY X
                        unimplemented!();
                    },
                    0xA => { // LDY
                        unimplemented!();
                    },
                    0xB => { // LDY X
                        unimplemented!();
                    },
                    0xC => { // CPY
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // CPX
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x5 => { // zpg
                match Self::high_nibble(instr) {
                    0x0 => { // ORA
                        unimplemented!();
                    },
                    0x1 => { // ORA X
                        unimplemented!();
                    },
                    0x2 => { // AND
                        unimplemented!();
                    },
                    0x3 => { // AND X
                        unimplemented!();
                    },
                    0x4 => { // EOR
                        unimplemented!();
                    },
                    0x5 => { // EOR X
                        unimplemented!();
                    },
                    0x6 => { // ADC
                        unimplemented!();
                    },
                    0x7 => { // ADC X
                        unimplemented!();
                    },
                    0x8 => { // STA
                        unimplemented!();
                    },
                    0x9 => { // STA X
                        unimplemented!();
                    },
                    0xA => { // LDA
                        unimplemented!();
                    },
                    0xB => { // LDA X
                        unimplemented!();
                    },
                    0xC => { // CMP
                        unimplemented!();
                    },
                    0xD => { // CMP X
                        unimplemented!();
                    },
                    0xE => { // SBC
                        unimplemented!();
                    },
                    0xF => { // SBC X
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x6 => { // zpg
                match Self::high_nibble(instr) {
                    0x0 => { // ASL
                        unimplemented!();
                    },
                    0x1 => { // ASL X
                        unimplemented!();
                    },
                    0x2 => { // ROL
                        unimplemented!();
                    },
                    0x3 => { // ROL X
                        unimplemented!();
                    },
                    0x4 => { // LSR
                        unimplemented!();
                    },
                    0x5 => { // LSR X
                        unimplemented!();
                    },
                    0x6 => { // ROR
                        unimplemented!();
                    },
                    0x7 => { // ROR X
                        unimplemented!();
                    },
                    0x8 => { // STX
                        unimplemented!();
                    },
                    0x9 => { // STX Y
                        unimplemented!();
                    },
                    0xA => { // LDX
                        unimplemented!();
                    },
                    0xB => { // LDX Y
                        unimplemented!();
                    },
                    0xC => { // DEC
                        unimplemented!();
                    },
                    0xD => { // DEC X
                        unimplemented!();
                    },
                    0xE => { // INC
                        unimplemented!();
                    },
                    0xF => { // INC X
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x7 => {
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // Illegal
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // Illegal
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // Illegal
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // Illegal
                        unimplemented!();
                    },
                    0xB => { // Illegal
                        unimplemented!();
                    },
                    0xC => { // Illegal
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // Illegal
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x8 => { // impl
                match Self::high_nibble(instr) {
                    0x0 => { // PHP
                        unimplemented!();
                    },
                    0x1 => { // CLC
                        unimplemented!();
                    },
                    0x2 => { // PLP
                        unimplemented!();
                    },
                    0x3 => { // SEC
                        unimplemented!();
                    },
                    0x4 => { // PHA
                        unimplemented!();
                    },
                    0x5 => { // CLI
                        unimplemented!();
                    },
                    0x6 => { // PLA
                        unimplemented!();
                    },
                    0x7 => { // SEI
                        unimplemented!();
                    },
                    0x8 => { // DEY
                        unimplemented!();
                    },
                    0x9 => { // TYA
                        unimplemented!();
                    },
                    0xA => { // TAY
                        unimplemented!();
                    },
                    0xB => { // CLV
                        unimplemented!();
                    },
                    0xC => { // INY
                        unimplemented!();
                    },
                    0xD => { // CLD
                        unimplemented!();
                    },
                    0xE => { // INX
                        unimplemented!();
                    },
                    0xF => { // SED
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0x9 => { // abs
                match Self::high_nibble(instr) {
                    0x0 => { // ORA
                        unimplemented!();
                    },
                    0x1 => { // ORA Y
                        unimplemented!();
                    },
                    0x2 => { // AND
                        unimplemented!();
                    },
                    0x3 => { // AND Y
                        unimplemented!();
                    },
                    0x4 => { // EOR
                        unimplemented!();
                    },
                    0x5 => { // EOR Y
                        unimplemented!();
                    },
                    0x6 => { // ADC
                        unimplemented!();
                    },
                    0x7 => { // ADC Y
                        unimplemented!();
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // STA Y
                        unimplemented!();
                    },
                    0xA => { // LDA
                        unimplemented!();
                    },
                    0xB => { // LDA Y
                        unimplemented!();
                    },
                    0xC => { // CMP
                        unimplemented!();
                    },
                    0xD => { // CMP Y
                        unimplemented!();
                    },
                    0xE => { // SBS
                        unimplemented!();
                    },
                    0xF => { // SBS Y
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0xA => {
                match Self::high_nibble(instr) {
                    0x0 => { // ASL A
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // ROL A
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // LSR A
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // ROR A
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // TXA
                        unimplemented!();
                    },
                    0x9 => { // TXS
                        unimplemented!();
                    },
                    0xA => { // TAX
                        unimplemented!();
                    },
                    0xB => { // TSX
                        unimplemented!();
                    },
                    0xC => { // DEX
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // NOP
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0xB => {
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // Illegal
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // Illegal
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // Illegal
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // Illegal
                        unimplemented!();
                    },
                    0xB => { // Illegal
                        unimplemented!();
                    },
                    0xC => { // Illegal
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // Illegal
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0xC => { // abs
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // BIT
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // JMP
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // JMP ind
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // STY
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // LDY
                        unimplemented!();
                    },
                    0xB => { // LDY X
                        unimplemented!();
                    },
                    0xC => { // CPY
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // CPX
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0xD => { // abs
                match Self::high_nibble(instr) {
                    0x0 => { // ORA
                        unimplemented!();
                    },
                    0x1 => { // ORA X
                        unimplemented!();
                    },
                    0x2 => { // AND
                        unimplemented!();
                    },
                    0x3 => { // AND X
                        unimplemented!();
                    },
                    0x4 => { // EOR
                        unimplemented!();
                    },
                    0x5 => { // EOR X
                        unimplemented!();
                    },
                    0x6 => { // ADC
                        unimplemented!();
                    },
                    0x7 => { // ADC X
                        unimplemented!();
                    },
                    0x8 => { // STA
                        unimplemented!();
                    },
                    0x9 => { // STA X
                        unimplemented!();
                    },
                    0xA => { // LDA
                        unimplemented!();
                    },
                    0xB => { // LDA X
                        unimplemented!();
                    },
                    0xC => { // CMP
                        unimplemented!();
                    },
                    0xD => { //  CMP X
                        unimplemented!();
                    },
                    0xE => { // SBC
                        unimplemented!();
                    },
                    0xF => { // SBC X
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0xE => {
                match Self::high_nibble(instr) {
                    0x0 => { // ASL
                        unimplemented!();
                    },
                    0x1 => { // ASL X
                        unimplemented!();
                    },
                    0x2 => { // ROL
                        unimplemented!();
                    },
                    0x3 => { // ROL X
                        unimplemented!();
                    },
                    0x4 => { // LSR
                        unimplemented!();
                    },
                    0x5 => { // LSR X
                        unimplemented!();
                    },
                    0x6 => { // ROR
                        unimplemented!();
                    },
                    0x7 => { // ROR X
                        unimplemented!();
                    },
                    0x8 => { // STX
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // LDX
                        unimplemented!();
                    },
                    0xB => { // LDX Y
                        unimplemented!();
                    },
                    0xC => { // DEC
                        unimplemented!();
                    },
                    0xD => { // DEC X
                        unimplemented!();
                    },
                    0xE => { // INC
                        unimplemented!();
                    },
                    0xF => { // INC X
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            0xF => {
                match Self::high_nibble(instr) {
                    0x0 => { // Illegal
                        unimplemented!();
                    },
                    0x1 => { // Illegal
                        unimplemented!();
                    },
                    0x2 => { // Illegal
                        unimplemented!();
                    },
                    0x3 => { // Illegal
                        unimplemented!();
                    },
                    0x4 => { // Illegal
                        unimplemented!();
                    },
                    0x5 => { // Illegal
                        unimplemented!();
                    },
                    0x6 => { // Illegal
                        unimplemented!();
                    },
                    0x7 => { // Illegal
                        unimplemented!();
                    },
                    0x8 => { // Illegal
                        unimplemented!();
                    },
                    0x9 => { // Illegal
                        unimplemented!();
                    },
                    0xA => { // Illegal
                        unimplemented!();
                    },
                    0xB => { // Illegal
                        unimplemented!();
                    },
                    0xC => { // Illegal
                        unimplemented!();
                    },
                    0xD => { // Illegal
                        unimplemented!();
                    },
                    0xE => { // Illegal
                        unimplemented!();
                    },
                    0xF => { // Illegal
                        unimplemented!();
                    },
                    _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

                }
            },
            _ => panic!("Error: low_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),
        }
    }

    
}


fn main() {
    let test: i16 = -2;
    let testt: u16 = 2;
    println!("test: {}", testt as i16 + test);
}
