// use std::mem::size_of;

// // Op code of chip-8 has a size of 2 bytes = 16 bits, so using a usgined 16 bit type in rust
type OpCode = u16;

// // Memory of the Chip-8  has access to 4k (4096 bytes) of RAM
// //Memory Map is how the memory is layed out for usage
// /*

// +---------------+= 0xFFF (4095) End of Chip-8 RAM
// |               |
// |               |
// |               |
// |               |
// |               |
// | 0x200 to 0xFFF|
// |     Chip-8    |
// | Program / Data|
// |     Space     |
// |               |
// |               |
// |               |
// +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
// |               |
// |               |
// |               |
// +---------------+= 0x200 (512) Start of most Chip-8 programs
// | 0x000 to 0x1FF|
// | Reserved for  |
// |  interpreter  |
// +---------------+= 0x000 (0) Start of Chip-8 RAM
// 0x000 to 0x1FF (0 to 511) the first 512 Bytes is for the Chip-8 Interpreter
//     0x050 to 0x09F inclusive (80 to 159)
//     0x050 to 0x0A0 exclusive end (80 to 160)
//         is used to store the built in fonts made out of asterik(*)
//             the font are 0 to f(hex) so; 0,1,2,3,4,5,6,7,8,9,a,b,c,d,e,f
// 0x200 to 0xFFF (512 to 4095) the remaining 3584 bytes is of a Chip-8 program
//  */
// type Memory = [u8; 4096];

// // 15 One Bye (8-bit) general purpose register from V0 to V14(VE 14 is E in hex), and 16th( V15 =   F) special register for the carry flag (VE) to store carrys when substracting
// type CpuRegisters = [u8; 16];

// // index Register
// type I = u16;

// // Program Counter; it stores the currently executing address;
// type PC = u16;

// // the Display Called Gfx for some reason
// // Pixel are drawin in XOR mode only gotta be (1,0)=1, (1,1)=0, (0,0)=0
// // if a pixel is set to zero we use VF register (the special one ) we set it; for collisions
// type Gfx = [u8; 64 * 32];

// // Delay Timer it count at 60hz
// // When the timer is non-zero it decrements at the 60hz rate when it reache 0 it deactivates.
// type DelayTimer = u8;

// // Sound Timer it counts at 60hz, the system buzzer sounds when the time reaches zero
// // other source says that when the buzzer is greater than zero it also decrement at 60hz rate,
// type SoundTimer = u8;

// //
// type Stack = [u16; 16];
// // the one say 16bit other 8 bit; using 16bit(2bytes) for now.
// type StackPointer = u16;

// // The Keypad one byte per key and key is 0-9 and A-F each key is one byte or 8 bit
// type KeyPad = [u8; 16];

use std::fs;

// // 60hz = 60 opcode a second.
use minifb::{Key, Window, WindowOptions};

struct Chip8 {
    memory: [u8; 4096],
    cpu_registers: [u8; 16], //called Vx where x is the index
    i: u16,
    pc: u16,
    display: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u16>,
    stack_pointer: u16,
    key_pad: [u8; 5],
}

impl Chip8 {
    fn initialize() -> Self {
        let mut chip = Self {
            memory: [0; 4096],
            cpu_registers: [0; 16],
            i: 0,
            pc: 0x200,
            display: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: Vec::with_capacity(16),
            stack_pointer: 0,
            key_pad: [0; 5],
        };
        let default_font = [
            [0xF0 as u8, 0x90, 0x90, 0x90, 0xF0],
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

        let mut curr = 0x050;
        for font in default_font {
            for ch in font {
                chip.memory[curr] = ch;
                curr += 1;
            }
        }
        chip
    }
    fn load_program(&mut self, program: Vec<u8>) {
        let mut curr = 0x200; //same as 512
        for opcode in program {
            self.memory[curr] = opcode;
            curr += 1;
        }
    }

    fn execute(&self) {}
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
fn main() {
    let mut chip8 = Chip8::initialize();

    let mut buffer: Vec<u32> = vec![0; 64 * 32];

    let program_bytes = fs::read("IBM_Logo.ch8").unwrap();
    chip8.load_program(program_bytes);

    let mut window = Window::new(
        "Test - ESC to exit",
        64,
        32,
        WindowOptions {
            borderless: false,
            resize: false,
            title: false,
            scale: minifb::Scale::X8,
            scale_mode: minifb::ScaleMode::Stretch,
            topmost: false,
            transparency: false,
            none: false,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let zero = [0xF0 as u8, 0x90, 0x90, 0x90, 0xF0];

    let mut curr = 0 as usize;
    let mut row = 0;
    for sp in zero {
        let l = sp.count_ones() + sp.count_zeros();

        for n in (0..l).rev() {
            print!("{}", sp >> n & 1);
            if (sp >> n & 1) == 1 {
                buffer[curr] = from_u8_rgb(0, 127, 255);
            }
            curr += 1;
        }
        println!();
        row += 1;
        curr = row * 64;
    }

    // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
    while window.is_open() {
        let opcode: OpCode = (chip8.memory[chip8.pc as usize] as u16) << 8
            | chip8.memory[chip8.pc as usize + 1] as u16;

        let first = (opcode & 0xF000) >> 12;
        let nnn = opcode & 0x0FFF;

        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        let kk = (opcode & 0x00FF) as u8;

        //nibble
        let n = (opcode & 0x000F) as u8;

        // let opcode = 0xAAFA;
        // let nnn = opcode & 0x0FFF;
        // println!("ERROR UNMACTCHED OPCODE FOR FIRST {:#06X}", nnn);
        // break;

        match first {
            0x0 => match nnn {
                0x0E0 => {
                    //Clear Screen
                    window
                        .update_with_buffer(&vec![0; 64 * 32], 64, 32)
                        .unwrap();
                    // Go to next op-code
                    chip8.pc += 2;
                }
                0x0EE => {
                    // Set the program counter PC to the address (value) at the top of the stack,
                    chip8.pc = chip8.stack.pop().unwrap();

                    // then decrease the stack pointer by one
                    chip8.stack_pointer -= 1;
                }
                _ => {
                    panic!("THIS SHOULD NOT BE IMPLMENTed");
                }
            },
            0x1 => {
                // JP addr
                // set Program Counter to nnn adress.
                chip8.pc = nnn;
            }

            0x2 => {
                // CALL addr
                // Increment Stack Pointer
                chip8.stack_pointer += 1;

                // push current PC(program counter) on top of the stack.
                chip8.stack.push(chip8.pc);

                // Set PC to nnn addr
                chip8.pc = nnn;
            }

            0x3 => {
                // SE Vx, byte
                // Compare Vx register at pos x to kk skip one insctruction +4 or as normal +2

                if chip8.cpu_registers[x as usize] == kk {
                    chip8.pc += 4;
                } else {
                    chip8.pc += 2;
                }
            }

            0x4 => {
                // SNE Vx, byte
                // Skip  CPU Register at x is not equal to kk. pc +4
                if chip8.cpu_registers[x as usize] != kk {
                    chip8.pc += 4;
                } else {
                    chip8.pc += 2;
                }
            }

            0x5 => {
                // SE Vx, Vy
                // Skip if CPU REGISTER at X  == CPU REGISTER AT Y  pc +4

                if chip8.cpu_registers[x as usize] == chip8.cpu_registers[y as usize] {
                    chip8.pc += 4;
                } else {
                    chip8.pc += 2;
                }
            }

            0x6 => {
                // 6xkk

                // LD Vx, byte
                //Set  the register at pos X to KK
                chip8.cpu_registers[x as usize] = kk;
                chip8.pc += 2;
            }
            0x7 => {
                // 7xkk - ADD Vx, byte
                // CPU_REGISTER at X add it to KK
                // What to do if it overflows
                chip8.cpu_registers[x as usize] = chip8.cpu_registers[x as usize].wrapping_add(kk);

                //increseas pc
                chip8.pc += 2;
            }

            0x8 => match n {
                0x0 => {
                    // 8xy0 - LD Vx, Vy
                    // Set CPU Register at x to  CPU REGISTER_ at Y
                    chip8.cpu_registers[x as usize] = chip8.cpu_registers[y as usize];

                    //increseas pc
                    chip8.pc += 2;
                }

                0x1 => {
                    // 8xy1 - OR Vx, Vy
                    // SET CPU REGISTER at x to regis[x] BITWISIE OR regis[y]
                    chip8.cpu_registers[x as usize] =
                        chip8.cpu_registers[x as usize] | chip8.cpu_registers[y as usize];

                    //incresea pc
                    chip8.pc += 2;
                }

                0x2 => {
                    // 8xy2 - AND Vx, Vy
                    // Set CPU REGISTER at X to Vx BITWISE AND Vy
                    chip8.cpu_registers[x as usize] =
                        chip8.cpu_registers[x as usize] & chip8.cpu_registers[y as usize];

                    //incresea pc
                    chip8.pc += 2;
                }

                0x3 => {
                    // 8xy3 - XOR Vx, Vy
                    // Set CPU REGISTER at X to Vx BITWISE EXCLUSIVE OR Vy
                    chip8.cpu_registers[x as usize] =
                        chip8.cpu_registers[x as usize] ^ chip8.cpu_registers[y as usize];

                    //incresea pc
                    chip8.pc += 2;
                }
                0x4 => {
                    // 8xy4 - ADD Vx, Vy
                    // It does and add and if it wraps(overflows) it will set VF (register at 15, of 0xF) to 1;
                    match chip8.cpu_registers[x as usize]
                        .checked_add(chip8.cpu_registers[y as usize])
                    {
                        Some(total) => {
                            chip8.cpu_registers[x as usize] = total;
                            chip8.cpu_registers[0xF as usize] = 0;
                        }
                        None => {
                            chip8.cpu_registers[x as usize] = chip8.cpu_registers[x as usize]
                                .wrapping_add(chip8.cpu_registers[y as usize]);
                            chip8.cpu_registers[0xF as usize] = 1;
                        }
                    }
                }
                0x5 => {
                    //8xy5 - SUB Vx, Vy
                    //Set Vx = Vx - Vy, set VF = NOT borrow.

                    if chip8.cpu_registers[x as usize] > chip8.cpu_registers[y as usize] {
                        chip8.cpu_registers[0xF] = 1;
                    } else {
                        chip8.cpu_registers[0xF] = 0;
                    }
                    chip8.cpu_registers[x as usize] = chip8.cpu_registers[x as usize]
                        .wrapping_sub(chip8.cpu_registers[y as usize]);
                }
                _ => {
                    panic!("ERROR UNMACTCHED OPCODE FOR FIRST {:#06X}", opcode);
                }
            },

            _ => {
                panic!("ERROR UNMACTCHED OPCODE FOR FIRST {:#06X}", opcode);
            }
        };

        // window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}
