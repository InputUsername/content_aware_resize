//! Energy functions determine the "energy" of a pixel
//! in an image in order to be able to find the seam
//! of pixels with minimal total energy.

/// Alias for an energy function. Energy functions take an image buffer, its width and height,
/// and pixel x/y coordinates, and return the energy value of the pixel.
pub trait EnergyFunction: Fn(&[u8], usize, usize, usize, usize) -> u32 {}
impl<T: Fn(&[u8], usize, usize, usize, usize) -> u32> EnergyFunction for T {}

fn abs_diff(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Returns the energy of a pixel based on the sum of squared
/// differences between the color channels of neighboring pixels.
pub fn basic(img: &[u8], w: usize, h: usize, x: usize, y: usize) -> u32 {
    let h1 = x.saturating_sub(1);
    let h2 = usize::min(x + 1, w - 1);
    let v1 = y.saturating_sub(1);
    let v2 = usize::min(y + 1, h - 1);

    let offset_h = 3 * y * w;
    let h1 = offset_h + 3 * h1;
    let h2 = offset_h + 3 * h2;
    let ph1 = &img[h1..h1 + 3];
    let ph2 = &img[h2..h2 + 3];

    let offset_v = 3 * x;
    let v1 = 3 * v1 * w + offset_v;
    let v2 = 3 * v2 * w + offset_v;
    let pv1 = &img[v1..v1 + 3];
    let pv2 = &img[v2..v2 + 3];

    let mut sum = 0;

    for i in 0..3 {
        let dh = u32::from(abs_diff(ph1[i], ph2[i]));
        let dv = u32::from(abs_diff(pv1[i], pv2[i]));

        sum += dh * dh;
        sum += dv * dv;
    }

    sum
}
