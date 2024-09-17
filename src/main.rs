mod display_buffer;
use core::panic;
use std::{
    fs,
    thread::sleep,
    time::{Duration, Instant},
    usize,
};

use display_buffer::DisplayBuffer;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_COLOR: u32 = 0;
const BG_COLOR: u32 = 120;

struct Chip8VM {
    memory: [u8; 4096],
    v_general_registers: [u8; 16],
    i_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
}

impl Chip8VM {
    fn new() -> Self {
        //program_counter = 0x200;
        let default_font = [
            [0xF0_u8, 0x90, 0x90, 0x90, 0xF0],
            [0x20, 0x60, 0x20, 0x20, 0x70],
            [0xF0, 0x10, 0xF0, 0x80, 0xF0],
            [0xF0, 0x10, 0xF0, 0x10, 0xF0],
            [0x90, 0x90, 0xF0, 0x10, 0x10],
            [0xF0, 0x80, 0xF0, 0x10, 0xF0],
            [0xF0, 0x80, 0xF0, 0x90, 0xF0],
            [0xF0, 0x10, 0x20, 0x40, 0x40],
            [0xF0, 0x90, 0xF0, 0x90, 0xF0],
            [0xF0, 0x90, 0xF0, 0x10, 0xF0],
            [0xF0, 0x90, 0xF0, 0x90, 0x90],
            [0xE0, 0x90, 0xE0, 0x90, 0xE0],
            [0xF0, 0x80, 0x80, 0x80, 0xF0],
            [0xE0, 0x90, 0x90, 0x90, 0xE0],
            [0xF0, 0x80, 0xF0, 0x80, 0xF0],
            [0xF0, 0x80, 0xF0, 0x80, 0x80],
        ];
        let mut chip = Chip8VM {
            memory: [0; 4096],
            v_general_registers: [0; 16],
            i_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0x200,
            stack_pointer: 15,
            stack: [0; 16],
        };
        let mut start_font = 0x50;
        for font in default_font {
            for ch in font {
                chip.memory[start_font] = ch;
                start_font += 1;
            }
        }
        chip
    }

    fn load(&mut self, program: Vec<u8>) {
        for (i, byte) in program.into_iter().enumerate() {
            self.memory[512 + i] = byte
        }
    }

    fn exec(&mut self, buffer: &mut DisplayBuffer, display: &mut minifb::Window) {
        // Fetch from PC
        let opcode: u16 = (self.memory[self.program_counter as usize] as u16) << 8
            | self.memory[self.program_counter as usize + 1] as u16;

        /*
        Varibles Used when decoding the 2 byte instruction:

            nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
            n or nibble - A 4-bit value, the lowest 4 bits of the instruction
            x - A 4-bit value, the lower 4 bits of the high byte of the instruction
            y - A 4-bit value, the upper 4 bits of the low byte of the instruction
            kk or byte - An 8-bit value, the lowest 8 bits of the instruction

        */

        let first = (opcode & 0xF000) >> 12;
        let nnn = opcode & 0x0FFF;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let kk = (opcode & 0x00FF) as u8;

        self.program_counter += 2;

        match first {
            0x0 => match nnn {
                0x0E0 => {
                    buffer.clear();
                    display
                        .update_with_buffer(&buffer.0, WIDTH, HEIGHT)
                        .unwrap();

                    //self.program_counter += 2;
                }
                _ => {
                    panic!("")
                }
            },
            0x1 => {
                // 1nn - JP addr
                // Jump to location nnn.
                // The interpreter sets the program counter to nnn.
                self.program_counter = nnn;
            }
            0x2 => {}
            0x3 => {}
            0x4 => {}
            0x5 => {}
            0x6 => {
                // 6xkk
                // Set Vx = kk
                // The interpreter puts the value kk into register Vx.
                self.v_general_registers[x as usize] = kk;

                //self.program_counter += 2;
            }
            0x7 => {
                // 7xkk - ADD Vx, byte
                // Set Vx = Vx + kk.
                // Adds the value kk to the value of register Vx, then stores the result in Vx.

                self.v_general_registers[x as usize] =
                    self.v_general_registers[x as usize].wrapping_add(kk);

                //self.program_counter += 2;
            }
            0x8 => {}
            0x9 => {}
            0xA => {
                // Annn - LD I, addr
                // Set I = nnn.
                // The value of register I is set to nnn.

                self.i_register = nnn;

                //self.program_counter += 2;
            }
            0xB => {}
            0xC => {}
            0xD => {
                println!("{:04X}", opcode);
                let mut bytes = vec![];
                println!("\t{}", n);
                println!("\t x:{}", x);
                println!("\t y:{}", y);
                let vx = self.v_general_registers[x as usize];
                let vy = self.v_general_registers[y as usize];

                for i in 0..n {
                    bytes.push(self.memory[self.i_register as usize + i as usize]);
                }

                self.v_general_registers[0xF_usize] =
                    buffer.xor_write(bytes, vx as usize % WIDTH, vy as usize % HEIGHT) as u8;

                display
                    .update_with_buffer(&buffer.0, WIDTH, HEIGHT)
                    .unwrap();

                //self.program_counter += 2;
            }
            0xE => {}
            0xF => {}
            _ => {
                panic!("WOOH")
            }
        }
        //self.program_counter += 2;
    }
}

// Create Notes for Row/Column Major Order
// X = J
// Y = I
// Cols = Width
// Rows = Heigh
//

fn main() {
    let mut vm = Chip8VM::new();
    let program = fs::read("IBM_Logo.ch8").unwrap();

    vm.load(program);
    let mut display = Window::new(
        "chip8-display",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X16,
            ..WindowOptions::default()
        },
    )
    .unwrap();
    display.set_target_fps(60);

    let mut buffer = DisplayBuffer(vec![120_u32; WIDTH * HEIGHT]);

    let zero_sprite: [u8; 5] = [0b11110000, 0b10010000, 0b10010000, 0b10010000, 0b11110000];

    buffer.xor_write(zero_sprite.to_vec(), 61, 0);

    let start_timer = Instant::now();
    let mut count = 0;
    while display.is_open() && !display.is_key_down(Key::Escape) {
        //display.update_with_buffer(&buffer.0, WIDTH, HEIGHT);
        // 1000milisec/700 = 1428.57 microsec
        sleep(Duration::from_micros(1300));
        vm.exec(&mut buffer, &mut display);
        //count += 1;
    }
}
