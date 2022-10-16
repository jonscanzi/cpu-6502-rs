


#[cfg(tests)]
mod ppu_tests {
    use super::*;


    #[test]
    fn vbl_timing_flag_is_accurate() {
        // Based on 'VBL FLag Timing' here https://wiki.nesdev.com/w/index.php/PPU_frame_timing

    }
}
use std::sync::OnceState;

use crate::cpu::datastructures::doubleword;

use super::cpu::datastructures::word;
struct PPU {
    // Hardware register
    ppu_ctrl: word, // NMI enable (V), PPU master/slave (P), sprite height (H), background tile select (B), sprite tile select (S), increment mode (I), nametable select (NN)
    ppu_mask: word, // color emphasis (BGR), sprite enable (s), background enable (b), sprite left column enable (M), background left column enable (m), greyscale (G)
    ppu_status: word, // vblank (V), sprite 0 hit (S), sprite overflow (O); read resets write pair for $2005/$2006
    oam_addr: word, // OAM read/write address
    oam_data: word, // OAM data read/write
    ppu_scroll: word, // fine scroll position (two writes: X scroll, Y scroll)
    ppu_addr: word, // PPU read/write address (two writes: most significant byte, least significant byte)
    ppu_data: word, // PPU data read/write
    oam_dma: word, // OAM DMA high address

    current_frame_cycle: u16,
    current_scanline: u16, // 0 to 262
    currentscanline_cycle: u16, // 0 to 340

    // Emulation state
    exec_data: StateReturnData,
    exec_function_num: usize,

    internal_read_buffer: word,

    ppu_addr_write_is_high: bool, // Default true
}

const NAMETABLE1: u8              = 0;
const NAMETABLE2: u8              = 1;
const VRAM_ADD_INCREMENT: u8      = 2;
const SPRITE_PATTERN_ADDR: u8     = 3;
const BACKROUND_PATTERN_ADDR: u8  = 4;
const SPRITE_SIZE: u8             = 5;
const MASTER_SLAVE_SELECT: u8     = 6;
const GENERATE_NMI: u8            = 7;

enum StateReturnCode {
    Done, // Tells the runner to go to the next step
    NotDone,
}
union StateReturnData {
    // Placeholders
    data1: u8,
    data2: i64,
}
struct StateReturn {
    ret: StateReturnCode,
    data: StateReturnData,
}

struct State {
    curr_func: fn(&mut PPU) -> StateReturn,
}

const PPU_EXEC_STEPS_LIST: [fn(&mut PPU) -> StateReturn; 2] = [PPU::background_step1, PPU::step2];
const PPU_NUM_SCANLINES_NTSC: u16 = 262;


impl PPU {
    // fn read(&self, address: doubleword) -> word {
    //     let mut ret: word = word::zero();
    //     match address.host_native_value() {
    //         0x2000..=0x2fff => {
    //             ret = self.internal_read_buffer;
    //             self.internal_read_buffer = self.read()
    //         }
    //     }
    // }

    // fn internal_nbuffered_read(&self, address: doubleword) -> word {

    // }

    fn write(&mut self, address: doubleword) {
        todo!("ppu write")
    }

    // fn x_scroll()

    fn background_step1(&mut self) -> StateReturn {

        // if (self.)

        match self.current_scanline {
            0..=19 => {

            },
            20 => {

            },
            21..=260 => {

            },
            261 => {

            }
            _ => {
                //error
            }
        }
        self.current_scanline = (self.current_scanline + 1) % (PPU_NUM_SCANLINES_NTSC);

        unimplemented!()
    }

    fn step2(&mut self) -> StateReturn {
        unimplemented!()
    }

    /// Advance a single clock cycle in the PPU execution
    fn exec_one_cycle(&mut self) {
        let ret = PPU_EXEC_STEPS_LIST[self.exec_function_num](self);
        match ret.ret {
            StateReturnCode::Done => self.exec_function_num = (self.exec_function_num + 1) % PPU_EXEC_STEPS_LIST.len(),
            StateReturnCode::NotDone => (),
        }
        self.exec_data = ret.data;
    }


    fn draw_scanline(&mut self) {
        
    }

    fn select_base_nametable(&self) -> doubleword {
        match self.ppu_addr.host_native_value() & 0b00000011 {
            0 => {
                doubleword::from(0x2000)
            }
            1 => {
                doubleword::from(0x2400)
            }
            2 => {
                doubleword::from(0x2800)
            }
            3 => {
                doubleword::from(0x2C00)
            }
            _ => {
                panic!("select_base_nametable got an impossible value");
            }
        } 
    }
}