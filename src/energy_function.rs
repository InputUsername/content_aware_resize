use crate::image::Image;

/// Trait alias for energy functions.
///
/// Energy functions take an [`Image`](crate::image::Image) and pixel
/// x/y coordinates, and return the energy value of that pixel.
///
/// This trait is automatically implemented for all such functions.
pub trait EnergyFunction: Fn(&Image, u32, u32) -> u32 {}

impl<F> EnergyFunction for F where F: Fn(&Image, u32, u32) -> u32 {}

fn abs_diff(a: u8, b: u8) -> u32 {
    let res = if a > b { a - b } else { b - a };
    res.into()
}

/// Returns the energy of a pixel based on the sum of squared
/// differences between the color channels of neighboring pixels.
pub fn basic(image: &Image, x: u32, y: u32) -> u32 {
    let (w, h) = (image.width(), image.height());

    let x1 = x.saturating_sub(1);
    let x2 = (x + 1).min(w - 1);

    let dh: u32 = image.get_pixel(x1, y).iter()
        .zip(image.get_pixel(x2, y).iter())
        .map(|(&a, &b)| abs_diff(a, b))
        .sum();

    let y1 = y.saturating_sub(1);
    let y2 = (y + 1).min(h - 1);

    let dv: u32 = image.get_pixel(x, y1).iter()
        .zip(image.get_pixel(x, y2).iter())
        .map(|(&a, &b)| abs_diff(a, b))
        .sum();

    dh * dh + dv * dv
}
