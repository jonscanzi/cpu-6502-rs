use regex::Regex;
use super::cpu::datastructures::word;
use super::cpu::datastructures::doubleword;
use super::cpu::datastructures::InstructionStream;
use super::cpu::datastructures::Push;
use std::fmt;

const TEST_ASSEMBLER: &str = "
LDA #$01
STA $0200
LDA #$05
STA $0201
LDA #$08
STA $0202
";



impl fmt::Display for InstructionStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut acc_string = String::with_capacity(self.stream.len() * 3);
        for byte in &self.stream {
            acc_string.push_str(&format!("{:X?}\n", byte.native_value()));
        }
        write!(f, "{}", acc_string)
    }
}

pub fn parse_program(_program: &str) {
    // TODO: remove debug code
    let debug_program = TEST_ASSEMBLER;
    let program = debug_program;

    let program = program.trim();

    let mut stream = InstructionStream::new();
    let instr_list = program.lines();
    for instr in instr_list {
        parse_line(instr, &mut stream);
    }
    println!("Resulting binary:\n{}", stream);

}

fn parse_line(line: &str, stream: &mut InstructionStream) {
    // let debug_line = "CPS $AA";


    let instr = ParsedInstruction::eval(line);
    instr.emit(stream);

}

/// Parse a single instruction, i.e. a line
// pub fn parse_line(line: &str) {
//     // lazy_static so it is only compiled once
//     lazy_static! {

//         // Opcode followed by operand
//         static ref RE: Regex = Regex::new(assembly_regexes[0]).unwrap();
//     }

//         let text = "CPS $4455";
//         let cap = RE.captures(text).unwrap();
//         println!("{:?}", cap);
//         match cap.get(1) {
//             Some(_) => {
//                 // emit_instr_bytes(&cap[1].to_string(), None);
//             }

//             None => panic!("Error: missing operand for instruction {:?}", cap[0].to_string())
//         }


// }

#[derive(Default, Debug)]
struct ParsedInstruction {
    instr: Option<Instructions>,
    operand: Option<AddrModes>,
}

impl ParsedInstruction {

    #[inline]
    fn eval_operation(op: &str) -> Instructions {
        let ret = match op {
            "ADC" => Instructions::ADC,
            "AND" => Instructions::AND,
            "ASL" => Instructions::ASL,
            "BCC" => Instructions::BCC,
            "BCS" => Instructions::BCS,
            "BEQ" => Instructions::BEQ,
            "BIT" => Instructions::BIT,
            "BMI" => Instructions::BMI,
            "BNE" => Instructions::BNE,
            "BPL" => Instructions::BPL,
            "BRK" => Instructions::BRK,
            "BVC" => Instructions::BVC,
            "BVS" => Instructions::BVS,
            "CLC" => Instructions::CLC,
            "CLD" => Instructions::CLD,
            "CLI" => Instructions::CLI,
            "CLV" => Instructions::CLV,
            "CMP" => Instructions::CMP,
            "CPX" => Instructions::CPX,
            "CPY" => Instructions::CPY,
            "DEC" => Instructions::DEC,
            "DEX" => Instructions::DEX,
            "DEY" => Instructions::DEY,
            "EOR" => Instructions::EOR,
            "INC" => Instructions::INC,
            "INX" => Instructions::INX,
            "INY" => Instructions::INY,
            "JMP" => Instructions::JMP,
            "JSR" => Instructions::JSR,
            "LDA" => Instructions::LDA,
            "LDX" => Instructions::LDX,
            "LDY" => Instructions::LDY,
            "LSR" => Instructions::LSR,
            "NOP" => Instructions::NOP,
            "ORA" => Instructions::ORA,
            "PHA" => Instructions::PHA,
            "PHP" => Instructions::PHP,
            "PLA" => Instructions::PLA,
            "PLP" => Instructions::PLP,
            "ROL" => Instructions::ROL,
            "ROR" => Instructions::ROR,
            "RTI" => Instructions::RTI,
            "RTS" => Instructions::RTS,
            "SBC" => Instructions::SBC,
            "SEC" => Instructions::SEC,
            "SED" => Instructions::SED,
            "SEI" => Instructions::SEI,
            "STA" => Instructions::STA,
            "STX" => Instructions::STX,
            "STY" => Instructions::STY,
            "TAX" => Instructions::TAX,
            "TAY" => Instructions::TAY,
            "TSX" => Instructions::TSX,
            "TXA" => Instructions::TXA,
            "TXS" => Instructions::TXS,
            "TYA" => Instructions::TYA,
            _ => panic!("Error: unrecognised operation {}", op),
        };
        ret
    }

    /// Convenience function to make the use of this feature much cleaner
    #[inline]
    fn parse_doubleword_hex(hex: &str) -> doubleword {
        doubleword::from(u16::from_str_radix(hex, 16).expect(&format!("Error: wrong format for hex value: {}", hex)))
    }

    #[inline]
    fn parse_word_hex(hex: &str) -> word {
        word::from(u8::from_str_radix(hex, 16).expect(&format!("Error: wrong format for hex value: {}", hex)))
    }

    fn eval_operand(op: &str) -> AddrModes {

        // TODO: for the love of everyting please replace indexes with a proper abstraction
        // TODO: remove debug portion (running every regex to make sure only one matches), maybe rewrite in functional
        // TODO: make regex array static / lazy_static
        let mut ret: Option<AddrModes> = None;
        for (idx, re) in OPERAND_REGEXES.iter().map(|r| Regex::new(r).unwrap()).enumerate() {

            ret = match re.captures(op) {
                Some(cap) => {
                    let operand = &cap[1].to_string();
                    if let Some(_) = ret {
                        panic!("Error: found 2 parsing of {} in eval_operand(), the second one was {}", op, OPERAND_REGEXES[idx]);
                    }
                    match idx {
                        //0 r"^\$([0-9A-F]{4})$",       // abs
                        //1 r"^\$([0-9A-F]{4}),X$",     // abs X
                        //2 r"^\$([0-9A-F]{4}),Y$",     // abs Y
                        //3 r"^#\$([0-9A-F]{2})$",      // imm or #
                        //4 r"^$",                       // impl
                        //5 r"^\(\$([0-9A-F]{4})\)$",   // ind
                        //6 r"^\(\$([0-9A-F]{2}),X\)$",   // ind X
                        //7 r"^\(\$([0-9A-F]{2})\),Y$",   // ind Y
                        //8 r"^\$([0-9A-F]{2})$",   // rel
                        0 => Some(AddrModes::Absolute(Self::parse_doubleword_hex(operand))),
                        1 => Some(AddrModes::AbsoluteX(Self::parse_doubleword_hex(operand))),
                        2 => Some(AddrModes::AbsoluteY(Self::parse_doubleword_hex(operand))),
                        3 => Some(AddrModes::Immediate(Self::parse_word_hex(operand))),
                        4 => Some(AddrModes::Implied),
                        5 => Some(AddrModes::Indirect(Self::parse_doubleword_hex(operand))),
                        6 => Some(AddrModes::IndirectX(Self::parse_word_hex(operand))),
                        7 => Some(AddrModes::IndirectY(Self::parse_word_hex(operand))),
                        8 => Some(AddrModes::Relative(Self::parse_word_hex(operand))),
                        _ => panic!("Errror in eval_operand()"),
                    }
                },
                None => ret,
            };
        }
        match ret {
            Some(r) => r,
            None => panic!("Error: unrecognised operand {}", op),
        }
    }
    
    fn eval(line: &str) -> Self {
        

        let re = Regex::new(SPLIT_REGEX).unwrap();
        let cap = re.captures(line);

        let (opc, op) = match cap {
            Some(cap) => {
                (Self::eval_operation(&cap[1].to_string()), Self::eval_operand(&cap[2].to_string()))
            },
            None => panic!("Error: unrecognised line {}", line),
        };

        println!("{:?},    {:?}", opc, op);

        
        Self {
            instr: Some(opc),
            operand: Some(op),
        }
        
    }

    /// Creates the little-endian binary representation of the instruction
    fn emit(self, stream: &mut InstructionStream) {

        let mut instr_byte = word::zero();

        let instr = self.instr.expect("Error: trying to eval() and assembly-parsed instruction without parsing an instruction first.");
        

        

        // TODO: (Optimisation) compact this into a single u8 / bit field
        
        
        // Illegal (instruction, operand) combination check, matrix style
        match (&instr, &self.operand) {
            // TODO: actually implement this
            _ => (), // Rest is considered legal
        }
        
        // Enconding rules taken from http://nparker.llx.com/a2/opcodes.html
        match instr {
            Instructions::ADC => {
                instr_byte.update_aaa(0b011);
                instr_byte.update_cc(0b01);
            },
            Instructions::AND => {
                instr_byte.update_aaa(0b001);
                instr_byte.update_cc(0b01);
            },
            Instructions::ASL => {
                instr_byte.update_aaa(0b000);
                instr_byte.update_cc(0b10);
            },
            Instructions::BCC => {
                instr_byte = word::from(0x90_u8);
            },
            Instructions::BCS => {
                instr_byte = word::from(0xB0_u8);
            },
            Instructions::BEQ => {
                instr_byte = word::from(0xF0_u8);
            },
            Instructions::BIT => {
                instr_byte.update_aaa(0b001);
                instr_byte.update_cc(0b00);
            },
            Instructions::BMI => {
                instr_byte = word::from(0x30u8);
            },
            Instructions::BNE => {
                instr_byte = word::from(0xD0_u8);
            },
            Instructions::BPL => {
                instr_byte = word::from(0x10_u8);
            },
            Instructions::BRK => {
                instr_byte = word::from(0x00_u8);
            },
            Instructions::BVC => {
                instr_byte = word::from(0x50_u8);
            },
            Instructions::BVS => {
                instr_byte = word::from(0x70_u8);

            },
            Instructions::CLC => {
                instr_byte.update_bbb(0b110);
                instr_byte.update_cc(0b00);
            },
            Instructions::CLD => {
                instr_byte.update_aaa(0b110);
                instr_byte.update_bbb(0b110);
            },
            Instructions::CLI => {
                instr_byte.update_aaa(0b010);
                instr_byte.update_bbb(0b110);
                instr_byte.update_cc(0b00);
            },
            Instructions::CLV => {
                instr_byte.update_aaa(0b101);
                instr_byte.update_bbb(0b110);
            },
            Instructions::CMP => {
                instr_byte.update_aaa(0b110);
                instr_byte.update_cc(0b01);
            },
            Instructions::CPX => {
                instr_byte.update_aaa(0b111);
                instr_byte.update_cc(0b00);
            },
            Instructions::CPY => {
                instr_byte.update_aaa(0b110);
                instr_byte.update_cc(0b00);
            },
            Instructions::DEC => {
                instr_byte.update_aaa(0b110);
                instr_byte.update_cc(0b10);
            },
            Instructions::DEX => {
                instr_byte.update_aaa(0b110);
                instr_byte.update_bbb(0b010);
                instr_byte.update_cc(0b10);
            },
            Instructions::DEY => {
                instr_byte = word::from(0x88_u8);
            },
            Instructions::EOR => {
                instr_byte.update_aaa(0b010);
                instr_byte.update_cc(0b01);
            },
            Instructions::INC => {
                instr_byte.update_aaa(0b111);
                instr_byte.update_cc(0b10);
            },
            Instructions::INX => {
                instr_byte = word::from(0xE8_u8);
            },
            Instructions::INY => {
                instr_byte = word::from(0xC8_u8);
            },
            Instructions::JMP => {
                instr_byte.update_aaa(0b010);
                instr_byte.update_cc(0b00);
            },
            Instructions::JSR => {
                instr_byte = word::from(0x20_u8);
            },
            Instructions::LDA => {
                instr_byte.update_aaa(0b101);
                instr_byte.update_cc(0b01);
            },
            Instructions::LDX => {
                instr_byte.update_aaa(0b101);
                instr_byte.update_cc(0b10);
            },
            Instructions::LDY => {
                instr_byte.update_aaa(0b101);
                instr_byte.update_cc(0b00);
            },
            Instructions::LSR => {
                instr_byte.update_aaa(0b010);
                instr_byte.update_cc(0b10);
            },
            Instructions::NOP => {
                instr_byte.update_aaa(0b111);
                instr_byte.update_bbb(0b010);
                instr_byte.update_cc(0b10);
            },
            Instructions::ORA => {
                instr_byte.update_aaa(0b000);
                instr_byte.update_cc(0b01);
            },
            Instructions::PHA => {
                instr_byte = word::from(0x48_u8);
            },
            Instructions::PHP => {
                instr_byte = word::from(0x08_u8);
            },
            Instructions::PLA => {
                instr_byte = word::from(0x68_u8);
            },
            Instructions::PLP => {
                instr_byte.update_aaa(0b001);
                instr_byte.update_bbb(0b010);
            },
            Instructions::ROL => {
                instr_byte.update_aaa(0b001);
                instr_byte.update_cc(0b10);
            },
            Instructions::ROR => {
                instr_byte.update_aaa(0b011);
                instr_byte.update_cc(0b10);
            },
            Instructions::RTI => {
                instr_byte = word::from(0x40_u8);
            },
            Instructions::RTS => {
                instr_byte = word::from(0x60_u8);
            },
            Instructions::SBC => {
                instr_byte.update_aaa(0b111);
                instr_byte.update_cc(0b01);
            },
            Instructions::SEC => {
                instr_byte.update_aaa(0b001);
                instr_byte.update_bbb(0b110);
                instr_byte.update_cc(0b00);
            },
            Instructions::SED => {
                instr_byte.update_aaa(0b111);
                instr_byte.update_bbb(0b110);
            },
            Instructions::SEI => {
                instr_byte.update_aaa(0b011);
                instr_byte.update_bbb(0b110);
            },
            Instructions::STA => {
                instr_byte.update_aaa(0b100);
                instr_byte.update_cc(0b01);
            },
            Instructions::STX => {
                instr_byte.update_aaa(0b100);
                instr_byte.update_cc(0b10);
            },
            Instructions::STY => {
                instr_byte.update_aaa(0b100);
                instr_byte.update_cc(0b00);
            },
            Instructions::TAX => {
                instr_byte = word::from(0xAA_u8);
            },
            Instructions::TAY => {
                instr_byte = word::from(0xA8_u8);
            },
            Instructions::TSX => {
                instr_byte = word::from(0xBA_u8);
            },
            Instructions::TXA => {
                instr_byte = word::from(0x8A_u8);
            },
            Instructions::TXS => {
                instr_byte = word::from(0x9A_u8);
            },
            Instructions::TYA => {
                instr_byte = word::from(0x98_u8);
            },
        }

        // Update bbb
        // TODO: call this in above match to make sure this doesn't change bbb for instructions
        //  that do not follow aaabbbcc pattern.

        if instr_byte.cc() == 0b01 {
            self.operand.clone().map(|operand| {
                // println!("DEBUG: making sure this piece of code is executed. Remove this line when sure.");
    
                match operand {
                    AddrModes::Absolute(_) => instr_byte.update_bbb(0b011_u8),
                    AddrModes::AbsoluteX(_) => instr_byte.update_bbb(0b111_u8),
                    AddrModes::AbsoluteY(_) => instr_byte.update_bbb(0b110_u8),
                    AddrModes::Immediate(_) => instr_byte.update_bbb(0b010_u8),
                    AddrModes::Implied => panic!("Impossible Instruction - operand combination"),
                    AddrModes::Indirect(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::IndirectX(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::IndirectY(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::Relative(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::Zeropage(_) => unimplemented!(),
                    AddrModes::ZeropageX(_) => unimplemented!(),
                    AddrModes::ZeropageY(_) => unimplemented!(),
                }
            });
        }

        else if instr_byte.cc() == 0b10 {
            self.operand.clone().map(|operand| {
                // println!("DEBUG: making sure this piece of code is executed. Remove this line when sure.");
    
                match operand {
                    AddrModes::Absolute(_) => instr_byte.update_bbb(0b011_u8),
                    AddrModes::AbsoluteX(_) => instr_byte.update_bbb(0b011_u8),
                    AddrModes::AbsoluteY(_) => instr_byte.update_bbb(0b011_u8),
                    AddrModes::Immediate(_) => instr_byte.update_bbb(0b000_u8),
                    AddrModes::Implied => panic!("Impossible Instruction - operand combination"),
                    AddrModes::Indirect(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::IndirectX(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::IndirectY(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::Relative(_) => panic!("Impossible Instruction - operand combination"),
                    AddrModes::Zeropage(_) => unimplemented!(),
                    AddrModes::ZeropageX(_) => unimplemented!(),
                    AddrModes::ZeropageY(_) => unimplemented!(),
                }
            });
        }
        
        stream.push(instr_byte);


        self.operand.map(|operand| {
            // println!("DEBUG: making sure this piece of code is executed. Remove this line when sure.");

            match operand {
                AddrModes::Absolute(dw) => stream.push(dw),
                AddrModes::AbsoluteX(dw) => stream.push(dw),
                AddrModes::AbsoluteY(dw) => stream.push(dw),
                AddrModes::Immediate(w) => stream.push(w),
                AddrModes::Implied => {},
                AddrModes::Indirect(dw) => stream.push(dw),
                AddrModes::IndirectX(w) => stream.push(w),
                AddrModes::IndirectY(w) => stream.push(w),
                AddrModes::Relative(w) => stream.push(w),
                AddrModes::Zeropage(w) => stream.push(w),
                AddrModes::ZeropageX(w) => stream.push(w),
                AddrModes::ZeropageY(w) => stream.push(w),
            }
        });
    }
}

#[derive(Debug, Clone)]
enum AddrModes {
    Absolute(doubleword),
    AbsoluteX(doubleword),
    AbsoluteY(doubleword),
    Immediate(word),
    Implied,
    Indirect(doubleword),
    IndirectX(word),
    IndirectY(word),
    Relative(word),
    Zeropage(word),
    ZeropageX(word),
    ZeropageY(word),
}

#[derive(Debug)]
enum Instructions {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}


const SPLIT_REGEX: &str = r"(\S+)\s*(.*)";
const OPERATION_REGEXES: &str = r"([A-Z]{3})";
const OPERAND_REGEXES: [&str; 9] = [
    r"^\$([0-9A-F]{4})$",       // abs
    r"^\$([0-9A-F]{4}),X$",     // abs X
    r"^\$([0-9A-F]{4}),Y$",     // abs Y
    r"^#\$([0-9A-F]{2})$",      // imm or #
    r"^$",                       // impl
    r"^\(\$([0-9A-F]{4})\)$",   // ind
    r"^\(\$([0-9A-F]{2}),X\)$",   // ind X
    r"^\(\$([0-9A-F]{2})\),Y$",   // ind Y
    r"^\$([0-9A-F]{2})$",   // rel
];


const ASSEMBLY_REGEXES: [&str; 9] = [
    r"^([A-Z]{3}) \$([0-9A-F]{4})$",       // abs
    r"^([A-Z]{3}) \$([0-9A-F]{4}),X$",     // abs X
    r"^([A-Z]{3}) \$([0-9A-F]{4}),Y$",     // abs Y
    r"^([A-Z]{3}) #\$([0-9A-F]{2})$",      // imm or #
    r"^([A-Z]{3})$",                       // impl
    r"^([A-Z]{3}) \(\$([0-9A-F]{4})\)$",   // ind
    r"^([A-Z]{3}) \(\$([0-9A-F]{2}),X\)$",   // ind X
    r"^([A-Z]{3}) \(\$([0-9A-F]{2})\),Y$",   // ind Y
    r"^([A-Z]{3}) \$([0-9A-F]{2})$",   // rel
    // FIXME: understand how to parse zpg addresses
    // r"^([A-Z]{3}) \(\$([0-9A-F]{4})\)$",   // zpg
    // r"^([A-Z]{3}) \(\$([0-9A-F]{4})\)$",   // zpg X
    // r"^([A-Z]{3}) \(\$([0-9A-F]{4})\)$",   // zpg Y
];



// #[inline]
// pub fn emit_instr_bytes(opcode: &str, operand: Option<&str>) {
//     match opcode {
//         "CPS" => println!("CPS"),
//         _ => panic!("Error: unrecognised opcode: {}", opcode),
//     }
// }
// #[derive(Copy, Clone)]
// enum OpParseState {
//     Idle,
//     AfterDollar,
//     Immediate,
//     ImmediateBytes,
//     XYIndex,
//     XIndex,
//     YIndex,
//     Indirect,
//     IndirectBytes,
//     ZeroPage,
//     DoneImmediate,
//     DoneXIndex,
//     DoneYIndex,
//     DoneDollar,
//     DoneIndirect,
// }

// #[derive(Copy, Clone)]
// enum XYIndexing {
//     None,
//     X,
//     Y
// }

// #[inline]
// // FSM style parsing
// fn parse_operand(operand: &str) -> AddrModes {

//     let mut state = OpParseState::Idle;
//     let mut indexing = XYIndexing::None;
//     let mut nibble_recorded = 0u8;
//     let mut reconstructed_value = "".to_string();

//     let test = operand.chars();

//     for chr in test {
//         if chr == '\n' {
//             break
//         }
//         if chr == ' ' {
//             continue
//         }
//         match state {
//             OpParseState::Idle => {
//                 match chr {
//                     '$' => {
//                         state = OpParseState::AfterDollar;
//                     }
//                     '#' => state = OpParseState::Immediate,
//                     '(' => state = OpParseState::Indirect,
//                     _ => unknown_char_error(operand),
//                 }
//             },
//             OpParseState::AfterDollar => {
//                 match chr {
//                     '0'..='9' | 'A'..='F' => match nibble_recorded {
//                         0..=2 => {
//                             reconstructed_value.push(chr);
//                             nibble_recorded+=1;
//                         },
//                         3 => {
//                             reconstructed_value.push(chr);
//                             nibble_recorded+=1;
//                             state = OpParseState::DoneDollar;
//                         }
//                         _ => unknown_char_error(operand),
//                     },
//                     ',' => state = OpParseState::XYIndex,
//                     _ => unknown_char_error(operand),
//                 }
//             },
//             OpParseState::XYIndex => {
//                 match chr {
//                     'Y' => state = OpParseState::YIndex,
//                     'X' => state = OpParseState::XIndex,
//                     _ => unknown_char_error(operand),
//                 }
//             },
//             OpParseState::Immediate => {
//                 match chr {
//                     '$' => state = OpParseState::ImmediateBytes,
//                     __ => unknown_char_error(operand),
//                 }
//             },
//             OpParseState::ImmediateBytes => {
//                 match chr {
//                     '0'..='9' | 'A'..='F' => match nibble_recorded {
//                         0..=2 => {
//                             reconstructed_value.push(chr);
//                             nibble_recorded+=1;
//                         },
//                         3 => {
//                             reconstructed_value.push(chr);
//                             nibble_recorded+=1;
//                             state = OpParseState::DoneImmediate;
//                         },
//                         _ => unknown_char_error(operand),
//                     },
//                     _ => unknown_char_error(operand),
//                 }
//             },

//             OpParseState::DoneDollar => match chr {
//                 ',' => 
//             },

//             _ => unimplemented!(),
            
//         }
//     }

    
    

//     unimplemented!();
// }

// #[inline]
// fn unknown_char_error(text: &str) {
//     panic!("Error: unrecognised character: {}", text);
// }