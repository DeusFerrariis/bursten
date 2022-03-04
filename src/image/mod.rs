mod pixel;

use bevy::prelude::*;

type Row = Vec<pixel::Pixel>;

pub struct Image {
    rows: Vec<Row>,
    height: u64,
    width: u64,
}

impl Image {
    fn new(height: u64, width: u64, color_type: pixel::ColorType) -> Self {
        let mut rows = Vec::new();
        let mut row = Vec::new();
        for x in 1..width as usize {
            row.push(pixel::Pixel::new(match color_type {
                pixel::ColorType::Rgb => pixel::Color::Rgb((0, 0, 0)),
                pixel::ColorType::Rgba => pixel::Color::Rgba((0, 0, 0, 0)),
            }));
        }

        for y in 1..height as usize {
            rows.push(row.clone());
        }

        Self { rows, height, width }
    }

    fn set(&mut self, x: u64, y: u64, pixel: pixel::Pixel) -> Result<(), ()> {
        if x > self.width || y > self.height {
            return Err(());
        }

        self.rows[y as usize][x as usize] = pixel;

        return Ok(());
    }

    fn get(&mut self, x: u64, y: u64) -> Option<pixel::Pixel> {
        if x > self.width || y > self.height {
            return None
        }

        return Some(self.rows[y as usize][x as usize])
    }

    fn rows(&self) -> Vec<Row> {
        return self.rows.clone();
    }
}
