use chip8::{
    cpu::{Chip8VM, HEIGHT, WIDTH},
    display_buffer::DisplayBuffer,
};
use std::{fs, time::Instant};

use minifb::{Key, Scale, Window, WindowOptions};

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

        for i in 0..16 {
            vm.keys[i] = display.is_key_down(btn_to_num(i));
        }

        vm.exec(&mut buffer);

        if vm.draw {
            vm.draw = false;
            println!("draw");
            display
                .update_with_buffer(&buffer.0, WIDTH, HEIGHT)
                .unwrap();
        }

        //count += 1;
    }
}

fn btn_to_num(i: usize) -> Key {
    match i {
        0x1 => Key::Key1,
        0x2 => Key::Key2,
        0x3 => Key::Key3,
        0xC => Key::Key4,

        0x4 => Key::Q,
        0x5 => Key::W,
        0x6 => Key::E,
        0xD => Key::R,

        0x7 => Key::A,
        0x8 => Key::S,
        0x9 => Key::D,
        0xE => Key::F,

        0xA => Key::Z,
        0x0 => Key::X,
        0xB => Key::C,
        0xF => Key::V,

        _ => Key::NumPadAsterisk,
    }
}
