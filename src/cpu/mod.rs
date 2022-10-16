
pub mod datastructures;
use datastructures::word;
use datastructures::doubleword;
use datastructures::ClAdd;
use datastructures::CARRY_BIT;
use datastructures::InstructionStream;

const RAM_SIZE_BYTES: usize = 0x800;


#[cfg(test)]
mod famicom_tests {


    use super::*;

    fn tests_init_system_resetted() -> System<FamicomMemory> {
        let mut ret = System::new_resetted();
        ret.pc = doubleword::from(0x8000u16); // FIXME: this is not famicom spec, change it when cpu is almost done
        ret
    }

    #[test]
    fn pc_can_advance_properly() {
        let mut sys = tests_init_system_resetted();
        sys.pc = doubleword::from(0u16);

        assert_eq!(sys.pc, 0u16);
        sys.advance_pc_1();
        assert_eq!(sys.pc, 1u16);
        sys.advance_pc_2();
        assert_eq!(sys.pc, 3u16);
        sys.advance_pc_3();
        assert_eq!(sys.pc, 6u16);
    }

    #[test]
    fn registers_reset_properly() {
        let mut sys = tests_init_system_resetted();

        sys.a = word::from(5u8);
        sys.x = word::from(5u8);
        sys.y = word::from(5u8);
        sys.s = word::from(5u8);
        sys.p = word::from(5u8);
        sys.pc = doubleword::from(5u16);

        sys.reset();

        assert_eq!(sys.a, 0u8);
        assert_eq!(sys.x, 0u8);
        assert_eq!(sys.y, 0u8);
        assert_eq!(sys.s, 0u8);
        assert_eq!(sys.p, 0u8);
        assert_eq!(sys.pc, 0u16);
    }

    #[test]
    fn mem_can_load_and_store() {
        let mut sys = tests_init_system_resetted();

        // Address 0 (min)
        let addr_min = doubleword::zero();
        assert_eq!(sys.load(addr_min), 0u8);
        sys.store(addr_min, word::from(1u8));
        assert_eq!(sys.load(addr_min), 1u8);

        // Address 4
        let addr_four = doubleword::from(4u16);
        assert_eq!(sys.load(addr_four), 0u8);
        sys.store(addr_four, word::from(1u8));
        assert_eq!(sys.load(addr_four), 1u8);

        // Max non-mirrored address 
        let addr_max_non_mirrored = doubleword::from(0x7FC);
        assert_eq!(sys.load(addr_max_non_mirrored), 0u8);
        sys.store(addr_max_non_mirrored, word::from(1u8));
        assert_eq!(sys.load(addr_max_non_mirrored), 1u8);

        // Max mirrored address 
        let addr_max_mirrored = doubleword::from(0x1FFC);
        assert_eq!(sys.load(addr_max_mirrored), 1u8);
        sys.store(addr_max_mirrored, word::from(0u8));
        assert_eq!(sys.load(addr_max_mirrored), 0u8);
    }


    #[test]
    fn cpu_can_load_and_store() {
        // Instruction sequence:
        // AND #00
        // ORA #01
        // STA 01

        let program: [u8; 6] = [0x29, 0x00, 0x09, 0x01, 0x85, 0x01];
        let program: Vec<word> = program.iter().map(|x| word::from(*x)).collect();

        let mut sys = tests_init_system_resetted();

        sys.run_programm_for(InstructionStream::from(program), 3);

        let addr = doubleword::from(0x01);
        assert_eq!(sys.load(addr), 0x01u8);


        // Instruction sequence:
        // AND #00
        // ORA #01
        // STA 01

        let program: [u8; 6] = [0x29, 0x00, 0x09, 0x01, 0x85, 0x01];
        let program: Vec<word> = program.iter().map(|x| word::from(*x)).collect();

        let mut sys = tests_init_system_resetted();

        sys.run_programm_for(InstructionStream::from(program), 3);

        let addr = doubleword::from(0x01);
        assert_eq!(sys.load(addr), 0x01u8);




        // Instruction sequence:
        // AND #00
        // ORA #01
        // STA 01

        let program: [u8; 6] = [0x29, 0x00, 0x09, 0x01, 0x85, 0x01];
        let program: Vec<word> = program.iter().map(|x| word::from(*x)).collect();

        let mut sys = tests_init_system_resetted();

        sys.run_programm_for(InstructionStream::from(program), 3);

        let addr = doubleword::from(0x01);
        assert_eq!(sys.load(addr), 0x01u8);



        // Instruction sequence:
        // AND #00
        // ORA #01
        // STA 01

        let program: [u8; 6] = [0x29, 0x00, 0x09, 0x01, 0x85, 0x01];
        let program: Vec<word> = program.iter().map(|x| word::from(*x)).collect();

        let mut sys = tests_init_system_resetted();

        sys.run_programm_for(InstructionStream::from(program), 3);

        let addr = doubleword::from(0x01);
        assert_eq!(sys.load(addr), 0x01u8);
    }


    #[test]
    fn low_nibble_test() {
        let val = 0x31u8;

        assert_eq!(val, 0x31u8);
        assert_eq!(low_nibble(word::from(val)), 0x1u8);
        assert_eq!(high_nibble(word::from(val)), 0x3u8);
    }


}

struct Ram {
    data: Box<[word; RAM_SIZE_BYTES]>,
}

impl Ram {

    /// Create a new NES-size RAM instance, all filled with zeroes
    fn new() -> Self {
        Self {
            data: Box::new([word::zero(); RAM_SIZE_BYTES]),
        }
    }

    fn write(&mut self, address: doubleword, data: word) {
        let address: u16 = address.host_native_value();
        debug_assert!(address <= 0x07ff);
        self.data[address as usize] = data;
    }

    fn read(&self, address: doubleword) -> word {
        debug_assert!(address.host_native_value() <= 0x07ff);
        self.data[address.as_addr()]
    }

    fn reset(&mut self) {
        self.data.iter_mut().for_each(|w| *w = word::zero())
    }
}

struct Cartridge {
    program: Box<[word; 0x8000]>,
}

impl Cartridge {
    pub fn new_zeroed() -> Self {
        Self {
            program: Box::new([word::zero(); 0x8000]),
        }
    }

    pub fn push_program(&mut self, program: InstructionStream) {
        let mut byte_idx = 0;
        for byte in program.stream {
            self.program[byte_idx] = byte;
            byte_idx+=1;
        }
    }

    pub fn read(&self, address: doubleword) -> word {
        self.program[address.as_addr() as usize]
    }
}

enum MemoryAccessType {
    Store,
    Load,
}

#[inline]
fn low_nibble(byte: word) -> u8 {
    //no need to worry about endianness since it's a single byte
    byte.host_native_value() & 0x0f
}

#[inline]
fn high_nibble(byte: word) -> u8 {
    (byte.host_native_value() & 0xf0) >> 4
}

/// Generic representation of the IO part of a system connected to a 6502 CPU
trait IO6502 {
    fn new_resetted() -> Self;

    fn reset(&mut self);

    fn push_program(&mut self, program: InstructionStream);

    fn store(&mut self, address: doubleword, data: word);

    fn load(&mut self, address: doubleword) -> word;
}

struct FamicomMemory {
    internal_ram: Ram,
    cart: Cartridge,
}

impl IO6502 for FamicomMemory {

    fn new_resetted() -> Self {

        let mut ret = Self {
            internal_ram: Ram::new(),
            cart: Cartridge::new_zeroed(),
        };

        ret.internal_ram.reset();
        ret
    }

    fn reset(&mut self) {
        self.internal_ram.reset();
    }

    fn push_program(&mut self, program: InstructionStream) {

        self.cart.push_program(program);

        
    }

    fn store(&mut self, address: doubleword, data: word) {
        let test_val = self.access(address, MemoryAccessType::Store, Some(data));
        debug_assert!(test_val.is_none());
    }

    fn load(&mut self, address: doubleword) -> word {
        self.access(address, MemoryAccessType::Load, None).expect("Reading memory failed.")
    }


}

impl FamicomMemory {
    fn access(&mut self, address: doubleword, tpe: MemoryAccessType, data: Option<word>) -> Option<word> {
        let addr = address.host_native_value();
        match addr {
            0x0000..=0x1FFF => {
                let real_address = address.host_native_value() % 0x800; // Clamp mirrored RAM addresses to the real ones
                match tpe {
                    MemoryAccessType::Load => Some(self.internal_ram.read(doubleword::from(real_address))),
                    MemoryAccessType::Store => {
                        self.internal_ram.write(doubleword::from(real_address), word::from(data.expect("access function got a store request without a value")));
                        None
                    },
                }
            }, // RAM (repeated)
            0x2000..=0x3FFF => None, // PPU (repeated)
            0x4000..=0x4017 => None, // APU and IO
            0x4018..=0x401F => None, // test Mode
            0x8000..=0xFFFF => { // cartridge
                match tpe {
                    MemoryAccessType::Load => Some(self.cart.read(doubleword::from(addr - 0x8000u16))),
                    MemoryAccessType::Store => panic!("Error: trying to make a store on the cartridge ROM"),
                }
            }, 
            _ => panic!(), // FIXME: complete range
        }
    }
}

struct System<T: IO6502> {
    a: word,
    x: word,
    y: word,
    /// Program counter
    pc: doubleword,
    /// Processor stack register
    s: word,
    /// Processor status register
    p: word,

    mem: T,
}

const C_BIT: u8 = 1 << 0;
const Z_BIT: u8 = 1 << 1;
const I_BIT: u8 = 1 << 2;
const D_BIT: u8 = 1 << 3;
const B_BIT: u8 = 1 << 4;
const V_BIT: u8 = 1 << 6;
const N_BIT: u8 = 1 << 7;

enum AddSubMode {
    Add,
    Sub,
}

impl<T: IO6502> System<T> {

    fn new_resetted() -> Self {

        let ret = Self {
            a: word::zero(),
            x: word::zero(),
            y: word::zero(),
            pc: doubleword::zero(),
            s: word::zero(),
            p: word::zero(),
            mem: T::new_resetted(),
        };
        ret
    }

    /// Run program only for a specific number of instructions before returning (mostly intended for debug)
    fn run_programm_for(&mut self, stream: InstructionStream, count: usize) {
        self.mem.push_program(stream);

        for x in 0..count {
            println!("instrr count: {}", x);
            self.advance_exec();
        }
    }


    fn run_program(&mut self, stream: InstructionStream) {
        self.mem.push_program(stream);
    }

    fn run(&mut self) {
        // General idea: fetch next instruction, execute it, wait a certain amount of time, start over

        loop {
            self.advance_exec();
            // sleep(1);
        }
    }

    #[inline]
    fn branch_on(&mut self, val: bool)  {
        match val {
            true => self.relative_jump(),
            false => self.advance_pc_2(),
        }
    }

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
    fn illegal_op(instr: word) {
        panic!("Error: illegal opcode {}", instr.cpu_endian_value());
    }

    #[inline]
    fn store(&mut self, address: doubleword, data: word) {
        self.mem.store(address, data);
    }

    #[inline]
    fn load(&mut self, address: doubleword) -> word {
        self.mem.load(address)
    }

    /// Load from memory, intepreting the value as a signed 16-bit address offset
    fn load_offset(&mut self, address: doubleword) -> i16 {
        self.mem.load(address).host_native_value() as i16

    }

    #[inline]
    fn load_doubleword(&mut self, address: doubleword) -> doubleword {
        let lo = self.mem.load(address);
        let hi = self.mem.load(address + 1u8);

        doubleword::from_words(hi, lo)
    }

    #[inline]
    fn advance_exec(&mut self) {
        let next_instr = self.mem.load(self.pc);
        self.alternate_exec(next_instr)
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

    #[inline]
    /// More comonly called 'pop'
    fn pull_word(&mut self) -> word {
        self.s = self.s - 1u8;
        self.load(self.s.as_doubleword())

    }

    #[inline]
    /// More comonly called 'pop'
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
    /// This function assumes that the jump offset is available at PC + 1
    fn relative_jump(&mut self) {
        self.pc = self.pc + self.load_offset(self.pc + 1u8);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn C(&self) -> bool {
        (self.p.host_native_value() & C_BIT) == C_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_C(&mut self) {
        self.p = word::from(self.p.host_native_value() | C_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_C(&mut self) {
        self.p = word::from(self.p.host_native_value() & !C_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_C(&mut self, val: bool) {
        match val {
            true => self.set_C(),
            false => self.clear_C(),
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn Z(&self) -> bool {
        (self.p.host_native_value() & Z_BIT) == Z_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_Z(&mut self) {
        self.p = word::from(self.p.host_native_value() | Z_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_Z(&mut self) {
        self.p = word::from(self.p.host_native_value() & !Z_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_Z(&mut self, val: bool) {
        match val {
            true => self.set_Z(),
            false => self.clear_Z(),
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn I(&self) -> bool {
        (self.p.host_native_value() & I_BIT) == I_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_I(&mut self) {
        self.p = word::from(self.p.host_native_value() | I_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_I(&mut self) {
        self.p = word::from(self.p.host_native_value() & !I_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_I(&mut self, val: bool) {
        match val {
            true => self.set_I(),
            false => self.clear_I(),
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn D(&self) -> bool {
        (self.p.host_native_value() & D_BIT) == D_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_D(&mut self) {
        self.p = word::from(self.p.host_native_value() | D_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_D(&mut self) {
        self.p = word::from(self.p.host_native_value() & !D_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_D(&mut self, val: bool) {
        match val {
            true => self.set_D(),
            false => self.clear_D(),
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn B(&self) -> bool {
        (self.p.host_native_value() & B_BIT) == B_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_B(&mut self) {
        self.p = word::from(self.p.host_native_value() | B_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_B(&mut self) {
        self.p = word::from(self.p.host_native_value() & !B_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_B(&mut self, val: bool) {
        match val {
            true => self.set_B(),
            false => self.clear_B(),
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn V(&self) -> bool {
        (self.p.host_native_value() & V_BIT) == V_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_V(&mut self) {
        self.p = word::from(self.p.host_native_value() | V_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_V(&mut self) {
        self.p = word::from(self.p.host_native_value() & !V_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_V(&mut self, val: bool) {
        match val {
            true => self.set_V(),
            false => self.clear_V(),
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    fn N(&self) -> bool {
        (self.p.host_native_value() & N_BIT) == N_BIT
    }

    #[inline]
    #[allow(non_snake_case)]
    fn set_N(&mut self) {
        self.p = word::from(self.p.host_native_value() | N_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn clear_N(&mut self) {
        self.p = word::from(self.p.host_native_value() & !N_BIT);
    }

    #[inline]
    #[allow(non_snake_case)]
    fn update_N(&mut self, val: bool) {
        match val {
            true => self.set_N(),
            false => self.clear_N(),
        }
    }

    // =============== HELPERS FUNCTIONS FOR RETRIEVING VALUES ===============

    #[inline]
    fn indirect_address(&mut self) -> doubleword {
        let (hi, lo) = (self.load(self.pc + 2u16), self.load(self.pc + 1u16));
        doubleword::from_words(hi, lo)
    }

    #[inline]
    /// Loads word from zeropage immediate + value at Y
    fn indirect_value(&mut self) -> word {
        let addr = self.indirect_address();
        let val = self.load(addr);
        val
    }

    #[inline]
    fn indirect_address_x(&mut self) -> doubleword {
        self.immediate_value().cl_add(self.x).as_doubleword()
    }

    #[inline]
    /// Used for ind addressing type with X
    fn indirect_value_x(&mut self) -> word {
        let addr = self.indirect_address_x();
        let val = self.load(addr);
        val
    }

    #[inline]
    /// Used for ind addressing type with Y
    fn indirect_value_y(&mut self) -> word {
        let addr = self.immediate_value();
        let val = self.load(addr.as_doubleword()) + self.y;
        val
    }

    #[inline]
    /// Convenience function to access # (immediate) value at pc + 1. Also used for zero-page addresses
    fn immediate_value(&mut self) -> word {
        self.load(self.pc + 1u8)
    }

    fn immediate_address(&mut self) -> doubleword {
        self.load(self.pc + 1u8).as_doubleword()
    }

    #[inline]
    /// Loads word from zeropage address + value at X
    fn zeropage_address(&mut self) -> doubleword {
        self.immediate_value().as_doubleword()
    }

    // TODO: check if X, Y offset in zero page is offset for address or for value at that address

    #[inline]
    /// Loads word from zeropage address + value at X
    fn zeropage_address_x(&mut self) -> doubleword {
        self.zeropage_address() + self.x.as_doubleword()
    }

    #[inline]
    /// Loads word from zeropage address + value at Y
    fn zeropage_address_y(&mut self) -> doubleword {
        self.zeropage_address() + self.x.as_doubleword()
    }

    #[inline]
    /// Used for loading the value for zpg instructions
    fn zeropage_value(&mut self) -> word {
        let addr = self.zeropage_address();
        self.load(addr)
    }

    #[inline]
    /// Loads word from zeropage immediate + value at X
    fn zeropage_value_x(&mut self) -> word {

        let addr = self.zeropage_address_x();
        self.load(addr)
    }

    #[inline]
    /// Loads word from zeropage immediate + value at Y
    fn zeropage_value_y(&mut self) -> word {
        let addr = self.zeropage_address_y();
        self.load(addr)
    }

    #[inline]
    fn relative_value(&mut self) -> doubleword {
        self.pc + (self.immediate_value().as_i16())
    }

    #[inline]
    fn update_flags_zn(&mut self, val: word) {

        let val = val.host_native_value_signed();

        if val < 0 {
            self.clear_Z();
            self.set_N();
        }
        else if val == 0 {
            self.set_Z();
            self.clear_N();
        }
        else { // val > 0
            self.clear_Z();
            self.clear_N();
        }
    }

    #[inline]
    /// 
    // FIXME: Test it
    fn absolute_address(&mut self) -> doubleword {
        let hi = self.load(self.pc + 2u16);
        let lo = self.load(self.pc + 1u16);
        doubleword::from_words(hi, lo)
    }

    #[inline]
    /// 
    fn absolute_value(&mut self) -> word {
        let addr = self.absolute_address();
        self.load(addr)
    }

    // =============== CONVENIENCE FUNCTIONS / MACROS FOR COMPUTATIONS ===============

    #[inline]
    /// Performs the 6502 compare operation: a substraction folowed by the updates of N, Z and C flags
    fn compare(&mut self, lhs: word, rhs: word) {
        // TODO: check if this correct, Flag behaviour is not clear yet
        let (res, did_overflow) = lhs.host_native_value().overflowing_sub(rhs.host_native_value());

        self.update_V(did_overflow);
        self.update_flags_zn(word::from(res));
    }

    #[inline]
    fn or(&mut self, lhs: word, rhs: word) -> word {
        let ret = lhs | rhs;
        self.update_flags_zn(ret);
        ret
    }

    #[inline]
    fn and(&mut self, lhs: word, rhs: word) -> word {
        let ret = lhs & rhs;
        self.update_flags_zn(ret);
        ret
    }

    #[inline]
    /// XOR
    fn eor(&mut self, lhs: word, rhs: word) -> word {
        let ret = lhs ^ rhs;
        self.update_flags_zn(ret);
        ret
    }

    #[inline]
    /// Same as EOR
    fn xor(&mut self, lhs: word, rhs: word) -> word {
        self.eor(lhs, rhs)
    }


    #[inline]
    /// Convencience function for all carry-type ops
    fn op_carry(&mut self, val: word, mode: AddSubMode) -> word {
        let c = self.C() as i16;
        let m = val.host_native_value_signed() as i16;
        let a = self.a.host_native_value_signed() as i16;

        let (res, did_overflow) = match mode {
            AddSubMode::Add => c.overflowing_add(m),
            AddSubMode::Sub => c.overflowing_sub(m),
        };
        let (res2, did_overflow_2) = match mode {
            AddSubMode::Add => res.overflowing_add(a),
            AddSubMode::Sub => res.overflowing_sub(a),
        };
        
        self.update_C((res2 & CARRY_BIT) != 0);
        self.update_V(did_overflow | did_overflow_2);
        let ret = word::from(res2);
        self.update_flags_zn(ret);
        ret
    }

    #[inline]
    /// ADC convenience function
    fn adc(&mut self, val: word) -> word {
        self.op_carry(val, AddSubMode::Add)
    }

    #[inline]
    /// ADC convenience function - equivalent to adc()
    fn add_carry(&mut self, val: word) -> word {
       self.adc(val)
    }

    #[inline]
    /// SBC convenience function
    fn sbc(&mut self, val: word) -> word {
        self.op_carry(val, AddSubMode::Sub)
    }

    #[inline]
    /// SBC convenience function - equivalent to sbc()
    fn sub_carry(&mut self, val: word) -> word {
        self.sbc(val)
    }

    #[inline]
    /// Convenience function for ASL, computes shift left and update N, Z and C flags
    fn asl(&mut self, val: word) -> word {
        let (ret, carry) = val.arith_shift_left_carry(1);
        self.update_flags_zn(ret);
        self.update_C(carry);
        ret
    }

    #[inline]
    fn lsr(&mut self, val: word) -> word {
        let (ret, carry) = val.logical_shift_right_carry(1);
        self.update_flags_zn(ret);
        self.update_C(carry);
        ret
    }

    #[inline]
    /// Performs ROL, update appropriate flags (N, Z, C) and returns result
    fn rol(&mut self, val: word) -> word {
        let (ret, carry) = val.rotate_left_carry(val, self.C());
        self.update_C(carry);
        self.update_flags_zn(ret);
        ret
    }

    #[inline]
    /// Performs ROR, update appropriate flags (N, Z, C) and returns result
    fn ror(&mut self, val: word) -> word {
        let (ret, carry) = val.rotate_right_carry(val, self.C());
        self.update_C(carry);
        self.update_flags_zn(ret);
        ret
    }

    // Re-implementation of decoding, using the aaabbbcc layout of opcodes
    fn alternate_exec(&mut self, instr: word) {
        
        let mut operand: Option<word> = None;
        let mut address: Option<doubleword>  = None;
        let mut pc_offset = 0u16;

        // Compute operand
        match instr.bbb() {
            0 => {
                match instr.cc() {
                    0 => {
                        match instr.aaa() {
                            1 => {
                                operand = Some(self.absolute_value());
                                address = Some(self.absolute_address());
                                pc_offset += 3;
                            },
                            5 | 6 | 7 => {
                                operand = Some(self.immediate_value());
                                address = Some(self.immediate_address());
                                pc_offset += 2;
                            },
                            _ => (),
                        }
                    }
                    1 => { // all X, ind
                        operand = Some(self.indirect_value_x());
                        address = Some(self.indirect_address_x());
                        pc_offset += 3;
                    }
                    2 => {
                        if instr.aaa() == 5 {
                            operand = Some(self.immediate_value());
                            address = Some(self.immediate_address());
                            pc_offset += 2;
                        }
                    }
                    _ => unimplemented!(),
                }
            },
            1 => { // zpg
                // TODO: some aaa, cc combinations should not do anything
                operand = Some(self.zeropage_value());
                address = Some(self.zeropage_address());
                pc_offset += 2;
            }

            2 => { // imm (#) or implied
                match instr.cc() {
                    1 => { // TODO: one aaa value should not do anything
                        operand = Some(self.immediate_value());
                        address = Some(self.immediate_address());
                        pc_offset += 2;
                    }, 
                    
                    _ => (),
                }
            }

            3 => { // abs (or ind)
                

                if instr.aaa() == 2 && instr.cc() == 0 { // only indirect instruction with b == 3
                    operand = Some(self.indirect_value());
                    address = Some(self.indirect_address());
                    pc_offset += 3;
                } else {
                    operand = Some(self.absolute_value());
                    address = Some(self.absolute_address());
                    pc_offset += 3;
                }
            }

            4 => {
                match instr.cc() {
                    1 => { // ind, Y
                        operand = Some(self.indirect_value_x());
                        address = Some(self.indirect_address_x());
                        pc_offset += 3;
                        unimplemented!();
                    }
                    _ => (),
                }
            }
            _ => unimplemented!(),
        }




        // operation itself
        match instr.cc() {
            0 => { // Lots of special instructions
                match instr.aaa() {
                    0 => {
                        match instr.bbb() {
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                            
                        }
                    },
                    1 => {
                        match instr.bbb() {
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            1 | 3 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    2 => {
                        match instr.bbb() {
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    3 => {
                        match instr.bbb() {
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    4 => {
                        match instr.bbb() {
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            1 | 3 | 5 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    5 => {
                        match instr.bbb() {
                            0 | 1 | 3 | 5 | 7 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    6 => {
                        match instr.bbb() {
                            0 | 1 | 3 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    7 => {
                        match instr.bbb() {
                            0 | 1 | 3 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            0 => { // INSTR_HERE
                                unimplemented!()
                            },
                            _ => unimplemented!(), // Illegal
                        }
                    },
                    _ => panic!("Error: word::aaa() gave a number greater than 7")
                }
            },


            1 => {
                match instr.aaa() {
                    0 => { // ORA
                        self.a = self.or(self.a, operand.unwrap());
                    },
                    1 => { // AND
                        self.and(self.a, operand.unwrap());
                    },
                    2 => { // EOR
                        self.eor(self.a, operand.unwrap());
                    },
                    3 => { // ADC
                        self.add_carry(operand.unwrap());
                    },
                    4 => { // STA
                        self.store(address.unwrap(), self.a);
                    },
                    5 => { // LDA
                        self.a = operand.unwrap();
                    },
                    6 => {
                        self.compare(self.a, operand.unwrap());
                    },
                    7 => {
                        self.sub_carry(operand.unwrap());
                    },
                    _ => panic!("Error: word::aaa() gave a number greater than 7")
                }
            }



            2 => {
                match instr.aaa() {
                    0 => { // ASL
                        unimplemented!();
                    },
                    1 => { // ROL
                        unimplemented!();
                    },
                    2 => { // LSR
                        unimplemented!();
                    },
                    3 => { // ROR
                        unimplemented!();
                    },
                    4 => {
                        match instr.bbb() {
                            1 | 3 | 5 => { // STX

                            },
                            2 => { // TXA
                                unimplemented!();
                            }
                            6 => { // TXS

                            },
                            _=> { // Illegal
                                unimplemented!();
                            }
                        }
                    },
                    5 => {

                        match instr.bbb() {
                            0 | 1 | 3 | 5 | 7 => { // LDX
                                self.x = operand.unwrap();
                            },
                            2 => { // TAX (unavoidable)
                                unimplemented!()
                            },
                            6 => { // TSX
                                unimplemented!()
                            }
                            _ => unimplemented!(),
                        }
                        
                    },
                    6 => {
                        match instr.bbb() {
                            1 | 3 | 5 | 7 => { // DEC
                                unimplemented!();
                            }
                            2 => { // DEX
                                unimplemented!();
                            }
                            _ => unimplemented!()  // Illegal
                        }
                    },
                    7 => {
                        match instr.bbb() {
                            1 | 3 | 5 | 7 => { // INC
                                unimplemented!()
                            },
                            2 => { // NOP
                                unimplemented!()
                            },
                            _ => unimplemented!() // Illegal
                        }
                    }
                    _ => panic!("Error: word::aaa() gave a number greater than 7")
                }
            }
            _ => unimplemented!(), // Illegal, c == 3 is never used in original 6502
        }
        self.pc = self.pc + pc_offset;
    }

    // fn exec(&mut self, instr: word) {
    //     // Matrix evaluation inspired by https://www.masswerk.at/6502/6502_instruction_set.html
    //     match low_nibble(instr) {
    //         0x0 => {
    //             match high_nibble(instr) {
    //                 0x0 => { // BRK
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // BPL
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // JSR
    //                     // Push (Next Instruction Address) -1 to stay canoncial with original implementation.
    //                     // RTS will then go back to that address + 1
    //                     self.push_doubleword(self.pc + 2u8); 
    //                     let lo = self.mem.load(self.pc + 1u8);
    //                     let hi = self.mem.load(self.pc + 2u8);
    //                     self.pc = doubleword::from_words(hi, lo);
    //                 },
    //                 0x3 => { // BMI
    //                     self.branch_on(self.N());
    //                 },
    //                 0x4 => { // RTI
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // BVC
    //                     self.branch_on(!self.V());
    //                 },
    //                 0x6 => { // RTS
    //                     self.pc = self.pull_doubleword();
    //                     self.advance_pc_1();
    //                 },
    //                 0x7 => { // BVS
    //                     self.branch_on(self.V());
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // BCC
    //                     self.branch_on(!self.C());
    //                 },
    //                 0xA => { // LDY #
    //                     let addr = self.load(self.pc + 1u8);
    //                     self.y = self.load(addr.as_doubleword());
    //                 },
    //                 0xB => { // BCS
    //                     self.branch_on(self.C());
    //                 },
    //                 0xC => { // CPY #
    //                     let rhs = self.load(self.pc + 1u8);
    //                     self.compare(self.y, rhs);
    //                     self.advance_pc_2();
    //                 },
    //                 0xD => { // BNE
    //                     self.branch_on(!self.Z());
    //                 },
    //                 0xE => { // CPX #
    //                     let rhs = self.load(self.pc + 1u8);
    //                     self.compare(self.x, rhs);
    //                     self.advance_pc_2();
    //                 },
    //                 0xF => { // BEQ rel
    //                     self.branch_on(self.Z());
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x1 => { // ind addressing
    //             // TODO: factor out common code
    //             match high_nibble(instr) {
    //                 0x0 => { // ORA X
    //                     let val = self.indirect_value_x();
    //                     self.a = self.a | val;
    //                     self.update_flags_zn(self.a);
    //                 },
    //                 0x1 => { // ORA Y
    //                     let val = self.indirect_value_y();
    //                     self.a = self.a | val;
    //                     self.update_flags_zn(self.a);
    //                 },
    //                 0x2 => { // AND X
    //                     let val = self.indirect_value_x();
    //                     self.a = self.a & val;
    //                     self.update_flags_zn(self.a);
    //                 },
    //                 0x3 => { // AND Y
    //                     let val = self.indirect_value_y();
    //                     self.a = self.a & val;
    //                     self.update_flags_zn(self.a);
    //                 },
    //                 0x4 => { // EOR X
    //                     let val = self.indirect_value_x();
    //                     self.a = self.a ^ val;
    //                     self.update_flags_zn(self.a);
    //                 },
    //                 0x5 => { // EOR Y
    //                     let val = self.indirect_value_y();
    //                     self.a = self.a ^ val;
    //                     self.update_flags_zn(self.a);
    //                 },
    //                 0x6 => { // ADC X
    //                     let val = self.indirect_value_x();
    //                     self.add_carry(val);
    //                 },
    //                 0x7 => { // ADC Y
    //                     let val = self.indirect_value_y();
    //                     self.add_carry(val);
    //                 },
    //                 0x8 => { // STA X
    //                     let addr = self.indirect_value_x().as_doubleword();
    //                     self.store(addr, self.a);
    //                 },
    //                 0x9 => { // STA Y
    //                     let addr = self.indirect_value_y().as_doubleword();
    //                     self.store(addr, self.a);
    //                 },
    //                 0xA => { // LDA X
    //                     let addr = self.indirect_value_x().as_doubleword();
    //                     self.a = self.load(addr);
    //                 },
    //                 0xB => { // LDA Y
    //                     let addr = self.indirect_value_y().as_doubleword();
    //                     self.a = self.load(addr);
    //                 },
    //                 0xC => { // CMP X
    //                     let val = self.indirect_value_x();
    //                     self.compare(self.a, val);
    //                 },
    //                 0xD => { // CMP Y
    //                     let val = self.indirect_value_y();
    //                     self.compare(self.a, val);
    //                 },
    //                 0xE => { // SBC X
    //                     let addr = self.indirect_value_x();
    //                     self.sbc(addr);
    //                 },
    //                 0xF => { // SBC Y
    //                     let addr = self.indirect_value_y();
    //                     self.sbc(addr);
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //             self.advance_pc_2();
    //         },
    //         0x2 => {
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // LDX #
    //                     self.x = self.immediate_value();
    //                     self.advance_pc_2();
    //                 },
    //                 0xB => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xC => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x3 => {
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xB => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xC => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x4 => { // zpg
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // BIT
    //                     let val = self.load(self.pc + 1u8);
    //                     let n_flag = (val & 0b10000000).native_value() != 0;
    //                     let v_flag = (val & 0b01000000).native_value() != 0;
    //                     self.update_N(n_flag);
    //                     self.update_V(v_flag);
    //                     let and = self.a & val;
    //                     self.update_Z(and.native_value() == 0);
    //                     self.advance_pc_2();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // STY zpg
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, self.y);
    //                     self.advance_pc_2();
    //                 },
    //                 0x9 => { // STY zpg X
    //                     let addr = self.zeropage_address_x();
    //                     self.store(addr, self.y);
    //                     self.advance_pc_2();
    //                 },
    //                 0xA => { // LDY zpg
    //                     let addr = self.zeropage_address();
    //                     self.y = self.load(addr);
    //                     self.update_flags_zn(self.y);
    //                     self.advance_pc_2();
    //                 },
    //                 0xB => { // LDY zpg X
    //                     let val = self.zeropage_value_x();
    //                     self.y = val;
    //                     self.update_flags_zn(self.y);
    //                     self.advance_pc_2();
    //                 },
    //                 0xC => { // CPY zpg
    //                     let val = self.zeropage_value();
    //                     self.compare(self.y, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // CPX zpg
    //                     let val = self.zeropage_value();
    //                     self.compare(self.x, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x5 => { // zpg
    //             match high_nibble(instr) {
    //                 0x0 => { // ORA
    //                     let val = self.zeropage_value();
    //                     self.a = self.or(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x1 => { // ORA X
    //                     let val = self.zeropage_value_x();
    //                     self.a = self.or(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x2 => { // AND
    //                     let val = self.zeropage_value();
    //                     self.a = self.and(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x3 => { // AND X
    //                     let addr = self.zeropage_value_x();
    //                     self.a = self.and(self.a, addr);
    //                     self.advance_pc_2();
    //                 },
    //                 0x4 => { // EOR
    //                     let val = self.zeropage_value();
    //                     self.a = self.eor(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x5 => { // EOR X
    //                     let val = self.zeropage_value_x();
    //                     self.a = self.eor(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x6 => { // ADC
    //                     let val = self.zeropage_value();
    //                     self.a = self.add_carry(val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x7 => { // ADC X
    //                     let val = self.zeropage_value_x();
    //                     self.a = self.add_carry(val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x8 => { // STA
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, self.a);
    //                     self.advance_pc_2();
    //                 },
    //                 0x9 => { // STA X
    //                     let addr = self.zeropage_address_x();
    //                     self.store(addr, self.a);
    //                     self.advance_pc_2();
    //                 },
    //                 0xA => { // LDA
    //                     let val = self.zeropage_value();
    //                     self.a = val;
    //                     self.advance_pc_2();
    //                 },
    //                 0xB => { // LDA X
    //                     let val = self.zeropage_value_x();
    //                     self.a = val;
    //                     self.advance_pc_2();
    //                 },
    //                 0xC => { // CMP
    //                     let val = self.zeropage_value();
    //                     self.compare(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0xD => { // CMP X
    //                     let val = self.zeropage_value_x();
    //                     self.compare(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0xE => { // SBC
    //                     let val = self.zeropage_value();
    //                     self.a = self.sub_carry(val);
    //                     self.advance_pc_2();
    //                 },
    //                 0xF => { // SBC X
    //                     let val = self.zeropage_value_x();
    //                     self.a = self.sub_carry(val);
    //                     self.advance_pc_2();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x6 => { // zpg
    //             // TODO: factor out common code here(e.g. self.zeropage_value())
    //             match high_nibble(instr) {
    //                 0x0 => { // ASL
    //                     let zpg = self.zeropage_value();
    //                     let new_val = self.asl(zpg);
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x1 => { // ASL X
    //                     let zpg = self.zeropage_value_x();
    //                     let new_val = self.asl(zpg);
    //                     let addr = self.zeropage_address_x();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x2 => { // ROL
    //                     let zpg = self.zeropage_value();
    //                     let new_val = self.rol(zpg);
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x3 => { // ROL X
    //                     let zpg = self.zeropage_value_x();
    //                     let new_val = self.rol(zpg);
    //                     let addr = self.zeropage_address_x();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x4 => { // LSR
    //                     let zpg = self.zeropage_value();
    //                     let new_val = self.lsr(zpg);
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x5 => { // LSR X
    //                     let zpg = self.zeropage_value_x();
    //                     let new_val = self.lsr(zpg);
    //                     let addr = self.zeropage_address_x();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x6 => { // ROR
    //                     let zpg = self.zeropage_value();
    //                     let new_val = self.ror(zpg);
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x7 => { // ROR X
    //                     let zpg = self.zeropage_value_x();
    //                     let new_val = self.ror(zpg);
    //                     let addr = self.zeropage_address_x();
    //                     self.store(addr, new_val);
    //                 },
    //                 0x8 => { // STX
    //                     let addr = self.zeropage_address();
    //                     self.store(addr, self.x);
    //                 },
    //                 0x9 => { // STX Y
    //                     let addr = self.zeropage_address_y();
    //                     self.store(addr, self.x);
    //                 },
    //                 0xA => { // LDX
    //                     self.x = self.zeropage_value();
    //                 },
    //                 0xB => { // LDX Y
    //                     self.x = self.zeropage_value_y();
    //                 },
    //                 0xC => { // DEC
    //                     let addr = self.zeropage_address();
    //                     let val = self.zeropage_value();
    //                     self.store(addr, val - 1);
    //                 },
    //                 0xD => { // DEC X
    //                     let addr = self.zeropage_address_x();
    //                     let val = self.zeropage_value_x();
    //                     self.store(addr, val - 1);
    //                 },
    //                 0xE => { // INC
    //                     let addr = self.zeropage_address();
    //                     let val = self.zeropage_value();
    //                     self.store(addr, val + 1);
    //                 },
    //                 0xF => { // INC X
    //                     let addr = self.zeropage_address_x();
    //                     let val = self.zeropage_value_x();
    //                     self.store(addr, val + 1);
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x7 => { // ALL ILLEGAL
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xB => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xC => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x8 => { // impl
    //             match high_nibble(instr) {
    //                 0x0 => { // PHP
    //                     self.push_word(self.p);
    //                 },
    //                 0x1 => { // CLC
    //                     self.clear_C();
    //                 },
    //                 0x2 => { // PLP
    //                     self.p = self.pull_word();
    //                 },
    //                 0x3 => { // SEC
    //                     self.set_C();
    //                 },
    //                 0x4 => { // PHA
    //                     self.push_word(self.a);
    //                 },
    //                 0x5 => { // CLI
    //                     self.clear_I();
    //                 },
    //                 0x6 => { // PLA
    //                     self.a = self.pull_word();
    //                 },
    //                 0x7 => { // SEI
    //                     self.set_I();
    //                 },
    //                 0x8 => { // DEY
    //                     self.y = self.y - 1;
    //                 },
    //                 0x9 => { // TYA
    //                     self.a = self.y;
    //                 },
    //                 0xA => { // TAY
    //                     self.y = self.a;
    //                 },
    //                 0xB => { // CLV
    //                     self.clear_V()
    //                 },
    //                 0xC => { // INY
    //                     self.y = self.y + 1;
    //                 },
    //                 0xD => { // CLD
    //                     // Unused on NES
    //                 },
    //                 0xE => { // INX
    //                     self.x = self.x + 1;
    //                 },
    //                 0xF => { // SED
    //                     // Unused on NES
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0x9 => { // abs
    //             match high_nibble(instr) {
    //                 0x0 => { // ORA #
    //                     let val = self.immediate_value();
    //                     self.a = self.or(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x1 => { // ORA Y
    //                     self.a = self.or(self.a, self.y);
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // AND #
    //                     let val = self.immediate_value();
    //                     self.a = self.and(self.a, val);
    //                     self.advance_pc_2();
    //                 },
    //                 0x3 => { // AND Y
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // EOR #
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // EOR Y
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // ADC #
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // ADC Y
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // STA Y
    //                     unimplemented!();
    //                 },
    //                 0xA => { // LDA #
    //                     unimplemented!();
    //                 },
    //                 0xB => { // LDA Y
    //                     unimplemented!();
    //                 },
    //                 0xC => { // CMP #
    //                     unimplemented!();
    //                 },
    //                 0xD => { // CMP Y
    //                     unimplemented!();
    //                 },
    //                 0xE => { // SBS #
    //                     unimplemented!();
    //                 },
    //                 0xF => { // SBS Y
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0xA => {
    //             match high_nibble(instr) {
    //                 0x0 => { // ASL A
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // ROL A
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // LSR A
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // ROR A
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // TXA
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // TXS
    //                     unimplemented!();
    //                 },
    //                 0xA => { // TAX
    //                     unimplemented!();
    //                 },
    //                 0xB => { // TSX
    //                     unimplemented!();
    //                 },
    //                 0xC => { // DEX
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // NOP
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0xB => {
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xB => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xC => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0xC => { // abs
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // BIT
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // JMP
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // JMP ind
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // STY
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // LDY
    //                     unimplemented!();
    //                 },
    //                 0xB => { // LDY X
    //                     unimplemented!();
    //                 },
    //                 0xC => { // CPY
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // CPX
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0xD => { // abs
    //             match high_nibble(instr) {
    //                 0x0 => { // ORA
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // ORA X
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // AND
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // AND X
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // EOR
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // EOR X
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // ADC
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // ADC X
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // STA
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // STA X
    //                     unimplemented!();
    //                 },
    //                 0xA => { // LDA
    //                     unimplemented!();
    //                 },
    //                 0xB => { // LDA X
    //                     unimplemented!();
    //                 },
    //                 0xC => { // CMP
    //                     unimplemented!();
    //                 },
    //                 0xD => { //  CMP X
    //                     unimplemented!();
    //                 },
    //                 0xE => { // SBC
    //                     unimplemented!();
    //                 },
    //                 0xF => { // SBC X
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0xE => {
    //             match high_nibble(instr) {
    //                 0x0 => { // ASL
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // ASL X
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // ROL
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // ROL X
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // LSR
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // LSR X
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // ROR
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // ROR X
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // STX
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // LDX
    //                     unimplemented!();
    //                 },
    //                 0xB => { // LDX Y
    //                     unimplemented!();
    //                 },
    //                 0xC => { // DEC
    //                     unimplemented!();
    //                 },
    //                 0xD => { // DEC X
    //                     unimplemented!();
    //                 },
    //                 0xE => { // INC
    //                     unimplemented!();
    //                 },
    //                 0xF => { // INC X
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         0xF => {
    //             match high_nibble(instr) {
    //                 0x0 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x1 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x2 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x3 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x4 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x5 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x6 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x7 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x8 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0x9 => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xA => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xB => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xC => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xD => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xE => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 0xF => { // Illegal
    //                     unimplemented!();
    //                 },
    //                 _ => panic!("Error: high_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),

    //             }
    //         },
    //         _ => panic!("Error: low_nibble() failed to convert to single hexadecimal number (i.e.) <= 0xF"),
    //     }
    // }

    
}