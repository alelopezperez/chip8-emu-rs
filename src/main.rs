use std::mem::size_of;

// Op code of chip-8 has a size of 2 bytes = 16 bits, so using a usgined 16 bit type in rust
type OpCode = u16;

// Memory of the Chip-8  has access to 4k (4096 bytes) of RAM
//Memory Map is how the memory is layed out for usage
/*

+---------------+= 0xFFF (4095) End of Chip-8 RAM
|               |
|               |
|               |
|               |
|               |
| 0x200 to 0xFFF|
|     Chip-8    |
| Program / Data|
|     Space     |
|               |
|               |
|               |
+- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
|               |
|               |
|               |
+---------------+= 0x200 (512) Start of most Chip-8 programs
| 0x000 to 0x1FF|
| Reserved for  |
|  interpreter  |
+---------------+= 0x000 (0) Start of Chip-8 RAM
0x000 to 0x1FF (0 to 511) the first 512 Bytes is for the Chip-8 Interpreter
    0x050 to 0x09F inclusive (80 to 159)
    0x050 to 0x0A0 exclusive end (80 to 160)
        is used to store the built in fonts made out of asterik(*)
            the font are 0 to f(hex) so; 0,1,2,3,4,5,6,7,8,9,a,b,c,d,e,f
0x200 to 0xFFF (512 to 4095) the remaining 3584 bytes is of a Chip-8 program
 */
type Memory = [u8; 4096];

// 15 One Bye (8-bit) general purpose register from V0 to V14(VE 14 is E in hex), and 16th( V15 =   F) special register for the carry flag (VE) to store carrys when substracting
type CpuRegisters = [u8; 16];

// index Register
type I = u16;

// Program Counter; it stores the currently executing address;
type PC = u16;

// the Display Called Gfx for some reason
// Pixel are drawin in XOR mode only gotta be (1,0)=1, (1,1)=0, (0,0)=0
// if a pixel is set to zero we use VF register (the special one ) we set it; for collisions
type Gfx = [u8; 64 * 32];

// Delay Timer it count at 60hz
// When the timer is non-zero it decrements at the 60hz rate when it reache 0 it deactivates.
type DelayTimer = u8;

// Sound Timer it counts at 60hz, the system buzzer sounds when the time reaches zero
// other source says that when the buzzer is greater than zero it also decrement at 60hz rate,
type SoundTimer = u8;

//
type Stack = [u16; 16];
// the one say 16bit other 8 bit; using 16bit(2bytes) for now.
type StackPointer = u16;

// The Keypad one byte per key and key is 0-9 and A-F each key is one byte or 8 bit
type KeyPad = [u8; 16];

// 60hz = 60 opcode a second.

struct Chip8 {
    memory: [u8; 4096],
    cpu_registers: [u8; 16],
    i: u16,
    pc: u16,
    display: [u8; 64 * 32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
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
            stack: [0x12; 16],
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
}
fn main() {
    let chip8 = Chip8::initialize();

    for r in 0..1 {}
}
