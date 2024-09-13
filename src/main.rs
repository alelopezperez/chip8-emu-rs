mod display_buffer;
use display_buffer::DisplayBuffer;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_COLOR: u32 = 0;
const BG_COLOR: u32 = 120;

struct Chip8VM {
    memory: [u8; 4096],
    general_registers: [u8; 16],
    i_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
}

impl Chip8VM {
    fn new(program: Vec<u8>) {}
}

// Create Notes for Row/Column Major Order
// X = J
// Y = I
// Cols = Width
// Rows = Heigh
//

fn main() {
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

    buffer.xor_write(zero_sprite.to_vec(), 61, 28);
    while display.is_open() && !display.is_key_down(Key::Escape) {
        display
            .update_with_buffer(&buffer.0, WIDTH, HEIGHT)
            .unwrap();
    }
}
