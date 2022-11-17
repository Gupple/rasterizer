use image::{RgbImage, Rgb};
use std::path::Path;

const LENGTH: u32 = 1920;
const WIDTH: u32 = 1080;

pub struct Canvas {
    canvas: image::RgbImage,
}

impl Canvas {
    pub fn new() -> Canvas {
        let mut canvas = Canvas{canvas: RgbImage::new(LENGTH, WIDTH)};
        canvas.clear();
        canvas
    }

    pub fn export(&self, filename: &str) {
        match self.canvas.save_with_format(
            Path::new(&format!("./exports/{filename}.png")),
            image::ImageFormat::Png) {
            Ok(_) => print!("Saved.\n"),
            Err(_) => print!("Not saved.\n"),
        };
    }

    pub fn place(&mut self, i: i32, j: i32, color: (u8, u8, u8)) {
        let i = (LENGTH as i32 / 2 + i) as u32;
        let j = (WIDTH as i32 / 2 + j) as u32;
        self.canvas.put_pixel(i, j, Rgb([color.0, color.1, color.2]));
    }

    pub fn clear(&mut self) {
        for i in 0..LENGTH {
            for j in 0..WIDTH {
                self.canvas.put_pixel(i, j, Rgb([255, 255, 255]));
            }
        }
    }
}

