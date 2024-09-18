use crate::display_buffer::DisplayBuffer;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_COLOR: u32 = 0;
const BG_COLOR: u32 = 120;

pub struct Chip8VM {
    memory: [u8; 4096],
    v_general_registers: [u8; 16],
    i_register: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 20],
}

impl Chip8VM {
    pub fn new() -> Self {
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
            stack: [0; 20],
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

    pub fn load(&mut self, program: Vec<u8>) {
        for (i, byte) in program.into_iter().enumerate() {
            self.memory[512 + i] = byte
        }
    }

    pub fn exec(&mut self, buffer: &mut DisplayBuffer, display: &mut minifb::Window) {
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
                0x0EE => {
                    // 00EE - RET
                    // Return from a subroutine.
                    // The interpreter sets the program counter to the address at the top of the stack,
                    // then subtracts 1 from the stack pointer.

                    self.program_counter = self.stack[self.stack_pointer as usize];
                    self.stack_pointer -= 1;
                }
                _ => {
                    panic!("");
                }
            },
            0x1 => {
                // 1nn - JP addr
                // Jump to location nnn.
                // The interpreter sets the program counter to nnn.

                self.program_counter = nnn;
            }
            0x2 => {
                // 2nnn - CALL addr
                // Call subroutine at nnn.
                // The interpreter increments the stack pointer,
                // then puts the current PC on the top of the stack. The PC is then set to nnn.

                self.stack_pointer += 1;
                self.stack[self.stack_pointer as usize] = self.program_counter;
                self.program_counter = nnn;
            }
            0x3 => {
                // 3xkk - SE Vx, byte
                // Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.

                if self.v_general_registers[x as usize] == kk {
                    self.program_counter += 2;
                }
            }
            0x4 => {
                // 4xkk - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.

                if self.v_general_registers[x as usize] != kk {
                    self.program_counter += 2;
                }
            }
            0x5 => {
                // 5xy0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.

                if self.v_general_registers[x as usize] == self.v_general_registers[y as usize] {
                    self.program_counter += 2;
                }
            }
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
            // Logical and arithmetic instructions
            0x8 => {
                // 8xyn
                match n {
                    0 => {
                        // 8xy0 - LD Vx, Vy
                        // Set Vx = Vy.
                        // Stores the value of register Vy in register Vx.

                        self.v_general_registers[x as usize] = self.v_general_registers[y as usize];
                    }
                    1 => {
                        // 8xy1 - OR Vx, Vy
                        // Set Vx = Vx OR Vy.
                        // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
                        // A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        self.v_general_registers[x as usize] |=
                            self.v_general_registers[y as usize];
                    }
                    2 => {
                        // 8xy2 - AND Vx, Vy
                        // Set Vx = Vx AND Vy.
                        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
                        // A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
                        self.v_general_registers[x as usize] &=
                            self.v_general_registers[y as usize];
                    }
                    3 => {
                        // 8xy3 - XOR Vx, Vy
                        // Set Vx = Vx XOR Vy.
                        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
                        // An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1.
                        // Otherwise, it is 0.

                        self.v_general_registers[x as usize] ^=
                            self.v_general_registers[y as usize];
                    }
                    4 => {
                        // 8xy4 - ADD Vx, Vy
                        // Set Vx = Vx + Vy, set VF = carry.
                        // The values of Vx and Vy are added together.
                        // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.

                        let add: u16 = self.v_general_registers[x as usize] as u16
                            + self.v_general_registers[y as usize] as u16;

                        if add > 255 {
                            self.v_general_registers[0xF] = 1
                        } else {
                            self.v_general_registers[0xF] = 0
                        }

                        self.v_general_registers[x as usize] = (add % 256) as u8;
                    }
                    5 => {
                        // 8xy5 - SUB Vx, Vy
                        // Set Vx = Vx - Vy, set VF = NOT borrow.
                        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.

                        if self.v_general_registers[x as usize]
                            > self.v_general_registers[y as usize]
                        {
                            self.v_general_registers[0xF] = 1;
                        } else {
                            self.v_general_registers[0xF] = 0;
                        }

                        self.v_general_registers[x as usize] = self.v_general_registers[x as usize]
                            .wrapping_sub(self.v_general_registers[y as usize]);
                    }
                    6 => {
                        // 8xy6 - SHR Vx {, Vy}
                        // Set Vx = Vx SHR 1.
                        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.

                        self.v_general_registers[0xF] =
                            self.v_general_registers[x as usize] & 0b00000001;

                        self.v_general_registers[x as usize] /= 2;
                        // self.v_general_registers[x as usize] >>= 1;
                    }
                    7 => {
                        // 8xy7 - SUBN Vx, Vy
                        // Set Vx = Vy - Vx, set VF = NOT borrow.
                        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.

                        if self.v_general_registers[y as usize]
                            > self.v_general_registers[x as usize]
                        {
                            self.v_general_registers[0xF] = 1
                        } else {
                            self.v_general_registers[0xF] = 0;
                        }

                        self.v_general_registers[x as usize] = self.v_general_registers[y as usize]
                            - self.v_general_registers[x as usize];
                    }
                    0xE => {
                        // 8xyE - SHL Vx {, Vy}
                        // Set Vx = Vx SHL 1.
                        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
                        //
                        self.v_general_registers[0xF] =
                            self.v_general_registers[x as usize] & 0b10000000;

                        self.v_general_registers[x as usize] *= 2;
                        // self.v_general_registers[x as usize] <<= 1;
                    }
                    _ => panic!("Eror in 0x8"),
                }
            }
            0x9 => {
                // 9xy0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.

                // TODO: pc = 2 * (Vx XOR VY)

                if self.v_general_registers[x as usize] == self.v_general_registers[y as usize] {
                    self.program_counter += 2
                }
            }
            0xA => {
                // Annn - LD I, addr
                // Set I = nnn.
                // The value of register I is set to nnn.

                self.i_register = nnn;
            }
            0xB => {
                /* WARNING:  THIS is ambiguos, BXNN is possible
                 */
                // JUMP WITH OFFSET
                // Bnnn - JP V0, addr
                // Jump to location nnn + V0.
                // The program counter is set to nnn plus the value of V0.
                self.program_counter = nnn + self.v_general_registers[0] as u16;
            }
            0xC => {
                // Cxkk - RND Vx, byte
                // Set Vx = random byte AND kk.
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
                // The results are stored in Vx. See instruction 8xy2 for more information on AND.

                self.v_general_registers[x as usize] = rand::random::<u8>() & kk;
            }

            0xD => {
                let mut bytes = vec![];

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
            // EX9E and EXA1: Skip if key
            0xE => match kk {
                0x9E => {
                    // Ex9E - SKP Vx
                    // Skip next instruction if key with the value of Vx is pressed.
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position,
                    // PC is increased by 2.
                    let is_key_down = display
                        .get_keys()
                        .iter()
                        .any(|&key| key as u8 == self.v_general_registers[x as usize]);

                    self.program_counter += 2 * is_key_down as u16;
                }
                0xA1 => {
                    // ExA1 - SKNP Vx
                    // Skip next instruction if key with the value of Vx is not pressed.
                    // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position,
                    // PC is increased by 2.

                    let is_key_down = !display
                        .get_keys()
                        .iter()
                        .any(|&key| key as u8 == self.v_general_registers[x as usize]);

                    self.program_counter += 2 * is_key_down as u16;
                }
                _ => {
                    panic!("ERROR in 9")
                }
            },
            0xF => {
                match kk {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        // Set Vx = delay timer value.
                        // The value of DT is placed into Vx.

                        self.v_general_registers[x as usize] = self.delay_timer;
                    }
                    0x0A => {
                        // Fx0A - LD Vx, K
                        // Wait for a key press, store the value of the key in Vx.
                        // All execution stops until a key is pressed, then the value of that key is stored in Vx.

                        while display.get_keys().is_empty() {}

                        let pressed_key = display.get_keys()[0] as u8;

                        self.v_general_registers[x as usize] = pressed_key;
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        // Set delay timer = Vx.
                        // DT is set equal to the value of Vx.

                        self.delay_timer = self.v_general_registers[x as usize];
                    }
                    0x18 => {
                        // Fx18 - LD ST, Vx
                        // Set sound timer = Vx
                        // ST is set equal to the value of Vx.

                        self.sound_timer = self.v_general_registers[x as usize];
                    }
                    0x1E => {
                        // Fx1E - ADD I, Vx
                        // Set I = I + Vx.
                        // The values of I and Vx are added, and the results are stored in I.

                        self.i_register += self.v_general_registers[x as usize] as u16;
                    }
                    0x29 => {
                        // Fx29 - LD F, Vx
                        // Set I = location of sprite for digit Vx.
                        // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
                        // See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

                        self.i_register = 0 + self.v_general_registers[x as usize] as u16;
                    }

                    0x33 => {
                        // Fx33 - LD B, Vx
                        // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
                        // the tens digit at location I+1, and the ones digit at location I+2.

                        let num = self.v_general_registers[x as usize];
                        let hundred = num / 100;
                        let ten = (num % 100) / 10;
                        let one = num % 10;

                        self.memory[self.i_register as usize] = hundred;
                        self.memory[self.i_register as usize + 1] = ten;
                        self.memory[self.i_register as usize + 2] = one;
                    }
                    0x55 => {
                        // Fx55 - LD [I], Vx
                        // Store registers V0 through Vx in memory starting at location I.
                        // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
                        for vx in 0..=x {
                            self.memory[self.i_register as usize + vx as usize] =
                                self.v_general_registers[vx as usize];
                        }
                    }
                    0x65 => {
                        // Fx65 - LD Vx, [I]
                        // Read registers V0 through Vx from memory starting at location I.
                        // The interpreter reads values from memory starting at location I into registers V0 through Vx.

                        for vx in 0..=x {
                            self.v_general_registers[vx as usize] =
                                self.memory[self.i_register as usize + vx as usize];
                        }
                    }

                    _ => {
                        panic!("panic on F")
                    }
                }
            }
            _ => {
                panic!("WOOH")
            }
        }
        //self.program_counter += 2;
    }
}
