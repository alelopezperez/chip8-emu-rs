use chip8::{
    cpu::{self, Chip8VM, HEIGHT, PIXEL_COLOR, WIDTH},
    display_buffer::DisplayBuffer,
};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
struct Emulator {
    cpu: Chip8VM,
    buffer: DisplayBuffer,
}

#[wasm_bindgen]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cpu: Chip8VM::new(),
            buffer: DisplayBuffer(vec![120_u32; WIDTH * HEIGHT]),
        }
    }

    #[wasm_bindgen]
    pub fn load(&mut self, data: Uint8Array) {
        let n = data.clone();
        self.cpu.load(n.to_vec());
    }

    #[wasm_bindgen]
    pub fn exec(&mut self) {
        self.cpu.exec(&mut self.buffer);
    }

    #[wasm_bindgen]
    pub fn can_draw(&self) -> bool {
        self.cpu.draw
    }

    #[wasm_bindgen]

    pub fn un_draw(&mut self) {
        self.cpu.draw = false;
    }

    #[wasm_bindgen]
    pub fn get_delay_timer(&self) -> u8 {
        self.cpu.delay_timer
    }

    #[wasm_bindgen]
    pub fn tick_delay_timer(&mut self) {
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, state: bool) {
        let key = evt.key();
        if let Some(which) = key2btn(&key) {
            self.cpu.keys[which] = state;
        }
    }

    #[wasm_bindgen]
    pub fn get_buffer(&self) -> Vec<u32> {
        let n_buff = self.buffer.clone();
        n_buff.0.clone()
    }
}
fn key2btn(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xC),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xD),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xE),
        "z" => Some(0xA),
        "x" => Some(0x0),
        "c" => Some(0xB),
        "v" => Some(0xF),
        _ => None,
    }
}
