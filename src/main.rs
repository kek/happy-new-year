extern crate imageproc;
extern crate minifb;

use image::{Rgb, RgbImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect::Rect,
};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rusttype::{Font, Scale};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut display_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let mut i = 0;
    let mut image_buffer = RgbImage::new(WIDTH as u32, HEIGHT as u32);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let colors = vec![
            Rgb([132u8, 132u8, 132u8]),
            Rgb([132u8, 255u8, 32u8]),
            Rgb([200u8, 255u8, 132u8]),
            Rgb([255u8, 0u8, 0u8]),
        ];
        let rand_num = rand::thread_rng().gen_range(0..colors.len());

        draw_filled_rect_mut(
            &mut image_buffer,
            Rect::at(100, 100).of_size(200, 100),
            colors[rand_num],
        );

        draw_filled_rect_mut(
            &mut image_buffer,
            Rect::at(100, 200).of_size(300, 100),
            Rgb([0, 0, 0]),
        );

        let font = font();
        let text = "Gott nytt år ".to_string() + &i.to_string();
        draw_text_mut(
            &mut image_buffer,
            Rgb([0, 255, 0]),
            100,
            200,
            Scale { x: 40.0, y: 40.0 },
            &font,
            &text,
        );
        if i == 2022 {
            i = 0;
        } else {
            i = i + 1;
        }

        let mut x = 0;
        let mut y = 0;
        for iter in display_buffer.iter_mut() {
            let color = image_buffer[(x, y)];
            *iter = color_value(color);
            x = x + 1;
            if x == WIDTH as u32 {
                x = 0;
                y = y + 1;
            }
        }

        window
            .update_with_buffer(&display_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn color_value(color: Rgb<u8>) -> u32 {
    let r = color[0] as u32;
    let g = color[1] as u32;
    let b = color[2] as u32;
    (r << 16) + (g << 8) + (b << 0)
}

fn font() -> Font<'static> {
    let ttf = Vec::from(include_bytes!("../assets/times.ttf") as &[u8]);
    Font::try_from_vec(ttf).unwrap()
}
