use crate::energy_function::EnergyFunction;
use crate::image::Image;

#[derive(Default, Clone)]
struct Energy {
    value: u32,
    backptr: usize,
}

pub struct EnergyMatrix(Vec<Energy>);

impl EnergyMatrix {
    pub fn new(image: &Image) -> Self {
        let (wu, hu) = (image.width() as usize, image.height() as usize);
        Self(vec![Energy::default(); wu * hu])
    }
}

pub fn find_vertical_seam<F: EnergyFunction>(
    image: &Image,
    energy_function: F,
    energy_matrix: &mut EnergyMatrix,
) -> Vec<usize> {
    let (w, h) = (image.width(), image.height());
    let (wu, hu) = (w as usize, h as usize);

    let matrix = &mut energy_matrix.0;

    for x in 0..w {
        matrix[x as usize] = Energy {
            value: energy_function(image, x, 0),
            backptr: 0,
        };
    }

    for y in 1..hu {
        for x in 0..wu {
            let lo = x.saturating_sub(1);
            let hi = (x + 1).min(wu - 1) + 1;

            let prev_row_start = (y - 1) * wu;
            let lo_idx = prev_row_start + lo;
            let hi_idx = prev_row_start + hi;

            let (prev_min, prev_min_x) = matrix[lo_idx..hi_idx]
                .iter()
                .zip(lo..hi)
                .min_by_key(|(e, _)| e.value)
                .unwrap();

            matrix[y * wu + x] = Energy {
                value: prev_min.value + energy_function(image, x as u32, y as u32),
                backptr: prev_min_x,
            }
        }
    }

    let last_row_start = (hu - 1) * wu;
    let mut seam_x_pos = matrix[last_row_start..]
        .iter()
        .enumerate()
        .min_by_key(|(_, e)| e.value)
        .map(|(x, _)| x)
        .unwrap();

    let mut seam = Vec::with_capacity(hu);

    for y in (0..hu).rev() {
        seam.push(seam_x_pos);
        seam_x_pos = matrix[y * wu + seam_x_pos].backptr;
    }

    seam
}

pub fn find_horizontal_seam<F: EnergyFunction>(
    image: &Image,
    energy_function: F,
    energy_matrix: &mut EnergyMatrix,
) -> Vec<usize> {
    let (w, h) = (image.width(), image.height());
    let (wu, hu) = (w as usize, h as usize);

    let matrix = &mut energy_matrix.0;

    for y in 0..h {
        matrix[y as usize] = Energy {
            value: energy_function(image, 0, y),
            backptr: 0,
        };
    }

    for x in 1..wu {
        for y in 0..hu {
            let lo = y.saturating_sub(1);
            let hi = (y + 1).min(hu - 1) + 1;

            let prev_col_start = (x - 1) * hu;
            let lo_idx = prev_col_start + lo;
            let hi_idx = prev_col_start + hi;

            let (prev_min, prev_min_y) = matrix[lo_idx..hi_idx]
                .iter()
                .zip(lo..hi)
                .min_by_key(|(e, _)| e.value)
                .unwrap();

            matrix[x * hu + y] = Energy {
                value: prev_min.value + energy_function(image, x as u32, y as u32),
                backptr: prev_min_y,
            };
        }
    }

    let last_col_start = (wu - 1) * hu;
    let mut seam_y_pos = matrix[last_col_start..]
        .iter()
        .enumerate()
        .min_by_key(|(_, e)| e.value)
        .map(|(y, _)| y)
        .unwrap();

    let mut seam = Vec::with_capacity(wu);

    for x in (0..wu).rev() {
        seam.push(seam_y_pos);
        seam_y_pos = matrix[x * hu + seam_y_pos].backptr;
    }

    seam
}
