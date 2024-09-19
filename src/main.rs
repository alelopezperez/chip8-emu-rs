use chip8::{cpu::Chip8VM, display_buffer::DisplayBuffer};
use std::{fs, time::Instant};

use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_COLOR: u32 = 0;
const BG_COLOR: u32 = 120;

// Create Notes for Row/Column Major Order
// X = J
// Y = I
// Cols = Width
// Rows = Heigh
//

fn main() {
    let mut vm = Chip8VM::new();
    let program = fs::read("pong.ch8").unwrap();

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
    while display.is_open() && !display.is_key_down(Key::Escape) {
        //display.update_with_buffer(&buffer.0, WIDTH, HEIGHT);
        // 1000milisec/700 = 1428.57 microsec
        //sleep(Duration::from_micros(1300));
        if start_timer.elapsed().as_millis() % 16 == 0 {
            if vm.delay_timer > 0 {
                vm.delay_timer -= 1;
            }

            if vm.sound_timer > 0 {
                vm.sound_timer -= 1;
            }
        }
        vm.exec(&mut buffer, &mut display);

        //count += 1;
    }
}
