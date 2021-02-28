use image::{DynamicImage, RgbImage, GrayImage};

use crate::energy_function::EnergyFunction;
use crate::seam::{EnergyMatrix, find_vertical_seam};

/// Represents a content-aware-resizable image.
pub struct Image {
    width: u32,
    height: u32,
    buffer: Vec<u8>,
}

impl Image {
    const CHANNELS: usize = 3;

    /// Create a new `Image` wrapper from an existing image.
    pub fn from_dynamic_image(image: DynamicImage) -> Self {
        let buffer = image.into_rgb8();
        Self {
            width: buffer.width(),
            height: buffer.height(),
            buffer: buffer.into_raw(),
        }
    }

    /// Convert `self` into an image buffer.
    pub fn into_image_buffer(self) -> RgbImage {
        RgbImage::from_raw(self.width(), self.height(), self.buffer).unwrap()
    }

    /// The width of this image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// The height of this image.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Retrieve the color components of the given pixel.
    pub fn get_pixel(&self, x: u32, y: u32) -> &[u8] {
        let (x, y) = (x as usize, y as usize);
        let wu = self.width() as usize;
        let idx = Self::CHANNELS * (y * wu + x);
        &self.buffer[idx..idx + Self::CHANNELS]
    }

    fn carve_vertical_seam<F: EnergyFunction>(
        &mut self,
        energy_function: F,
        energy_matrix: &mut EnergyMatrix,
    ) {
        let seam = find_vertical_seam(self, energy_function, energy_matrix);

        let (wu, hu) = (self.width() as usize, self.height() as usize);

        for y in 0..hu {
            let row_start = Self::CHANNELS * y * (wu - 1);
            let x_idx = row_start + Self::CHANNELS * seam[hu - 1 - y];
            let next_row_start = Self::CHANNELS * (y + 1) * (wu - 1);

            self.buffer.copy_within(x_idx + Self::CHANNELS..next_row_start, x_idx);
        }
    }

    /// Resize this image horizontally based on an energy function.
    pub fn resize_horizontal<F: EnergyFunction + Copy>(
        &mut self,
        new_width: u32,
        energy_function: F,
    ) {
        let mut energy_matrix = EnergyMatrix::new(self);

        for cur_width in (new_width..self.width()).rev() {
            self.carve_vertical_seam(energy_function, &mut energy_matrix);
            self.width = cur_width;
        }

        let (wu, hu) = (self.width() as usize, self.height() as usize);
        self.buffer.truncate(Self::CHANNELS * wu * hu);
        self.buffer.shrink_to_fit();
    }

    /// Dump the pixel energy values of this image to a grayscale image.
    pub fn energy<F: EnergyFunction>(&self, energy_function: F) -> GrayImage {
        let mut min = energy_function(self, 0, 0);
        let mut max = min;

        let (w, h) = (self.width(), self.height());

        let energies: Vec<f32> = (0..(w * h))
            .map(|i| (i % w, i / w))
            .map(|(x, y)| {
                let e = energy_function(self, x, y);
                min = e.min(min);
                max = e.max(max);
                e as f32
            })
            .collect();

        let min = min as f32;
        let max = max as f32;

        let output: Vec<u8> = energies
            .into_iter()
            .map(|e| (255.0 * (e - min) / (max - min)) as u8)
            .collect();

        GrayImage::from_raw(w, h, output).unwrap()
    }
}
