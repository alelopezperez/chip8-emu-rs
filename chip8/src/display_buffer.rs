const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const PIXEL_COLOR: u32 = 0;
const BG_COLOR: u32 = 120;

#[derive(Clone)]
pub struct DisplayBuffer(pub Vec<u32>);

impl DisplayBuffer {
    pub fn xor_write(&mut self, bytes: Vec<u8>, x: usize, y: usize) -> bool {
        println!(" HEYYY {x} {y}");
        let pixel_vec = bytes_to_pixels(bytes);
        let mut collision = false;

        for (y_offset, row) in pixel_vec.into_iter().enumerate() {
            let n_y = y + y_offset;
            if n_y >= HEIGHT {
                break;
            }
            for (x_offset, pixel) in row.into_iter().enumerate() {
                //let row_major_order_pos = (y + y_offset) * WIDTH + (x + x_offset);
                let n_x = x + x_offset;
                if n_x >= WIDTH {
                    continue;
                }
                let row_major_order_pos = (n_y) * WIDTH + (n_x);

                if self.0[row_major_order_pos] == pixel {
                    self.0[row_major_order_pos] = BG_COLOR;
                    if pixel == PIXEL_COLOR {
                        collision = true;
                    }
                } else {
                    self.0[row_major_order_pos] = PIXEL_COLOR;
                }
            }
        }

        collision
    }

    pub fn clear(&mut self) {
        self.0 = vec![BG_COLOR; WIDTH * HEIGHT];
    }
}
fn bytes_to_pixels(mut bytes: Vec<u8>) -> Vec<[u32; 8]> {
    bytes
        .iter_mut()
        .map(|byte| {
            let pixel_row: [u32; 8] = (0..8)
                .map(|i| {
                    let pixel = *byte & 0b10000000;
                    *byte <<= 1;

                    if pixel == 0b10000000 {
                        PIXEL_COLOR
                    } else {
                        BG_COLOR
                    }
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            pixel_row
        })
        .collect::<Vec<_>>()
}
