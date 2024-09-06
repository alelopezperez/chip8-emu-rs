use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

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

struct DisplayBuffer(Vec<u32>);

// Create Notes for Row/Column Major Order
// X = J
// Y = I
// Cols = Width
// Rows = Heigh
impl DisplayBuffer {
    fn update_buffer(&mut self, bytes: Vec<u8>, x: usize, y: usize) {
        let row_major_order_pos = y * WIDTH + x;
        self.0[row_major_order_pos] = 0;
    }
}

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

    while display.is_open() && !display.is_key_down(Key::Escape) {
        buffer.update_buffer(vec![0, 1], 0, 0);
        buffer.update_buffer(vec![0, 1], 1, 1);
        display
            .update_with_buffer(&buffer.0, WIDTH, HEIGHT)
            .unwrap();
    }
}
