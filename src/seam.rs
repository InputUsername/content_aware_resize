use copy_in_place::copy_in_place;

#[derive(Clone, Copy)]
pub struct Energy {
    value: u32,
    backptr: usize
}

pub fn make_energy_buffer(w: usize, h: usize) -> Vec<Energy> {
    Vec::with_capacity(w * h)
}

/// Finds the (vertical) minimal energy seam as determined
/// by `energy_function`. The resulting vector will contain
/// the horizontal positions of the pixels making up the seam,
/// in reverse order (bottom to top) of the image.
pub fn find_min_energy_seam<F>(img: &[u8], w: usize, h: usize, energy_function: F, energy_buffer: &mut Vec<Energy>) -> Vec<usize>
    where F: Fn(&[u8], usize, usize, usize, usize) -> u32
{
    for x in 0..w {
        energy_buffer.push(Energy {
            value: energy_function(img, w, h, x, 0),
            backptr: 0
        });
    }

    for y in 1..h {
        for x in 0..w {
            let lo = x.saturating_sub(1);
            let hi = usize::min(x + 1, w - 1) + 1;

            let prev_row_start = (y - 1) * w;
            let lo_idx = prev_row_start + lo;
            let hi_idx = prev_row_start + hi;

            let (prev_min, prev_min_x) = energy_buffer[lo_idx .. hi_idx].iter()
                .zip(lo..hi)
                .min_by_key(|(e, _)| e.value)
                .unwrap();
            let prev_min_value = prev_min.value;

            energy_buffer.push(Energy {
                value: prev_min_value + energy_function(img, w, h, x, y),
                backptr: prev_min_x
            });
        }
    }

    let mut seam = Vec::with_capacity(h);

    let last_row_start = (h - 1)*w;
    let (mut seam_x_pos, _) = energy_buffer[last_row_start..].iter()
        .enumerate()
        .min_by_key(|(_, e)| e.value)
        .unwrap();

    for y in (0..h).rev() {
        seam.push(seam_x_pos);
        seam_x_pos = energy_buffer[y * w + seam_x_pos].backptr;
    }

    seam
}

/// Finds and removes the (vertical) minimal energy seam as determined
/// by `energy_function`; the resulting image will be contained in the
/// first `(w - 1) * h` pixels of `img`. Afterwards the container can
/// be safely resized to `(w - 1) * h` without losing data.
pub fn remove_min_energy_seam<F>(img: &mut [u8], w: usize, h: usize, energy_function: F, energy_buffer: &mut Vec<Energy>)
    where F: Fn(&[u8], usize, usize, usize, usize) -> u32
{
    let seam = find_min_energy_seam(img, w, h, energy_function, energy_buffer);

    for y in 0..h {
        let row_start = 3 * y * (w - 1);
        let x_idx = row_start + 3 * seam[h - 1 - y];

        assert!(x_idx <= 3 * w * h);

        // move all pixels after the seam one position to the left
        copy_in_place(img, (x_idx + 3) .. (3 * w * h - 3 * y), x_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENERGIES: &[&[u32]] = &[
        &[9, 9, 0, 9, 9],
        &[9, 1, 9, 8, 9],
        &[9, 9, 9, 9, 0],
        &[9, 9, 9, 0, 9]
    ];
    const W: usize = 5;
    const H: usize = 4;

    fn dummy_energy(_img: &[u8], _w: usize, _h: usize, x: usize, y: usize) -> u32 {
        ENERGIES[y][x]
    }

    #[test]
    fn test_find_min_energy_seam() {
        let mut buffer = make_energy_buffer(W, H);
        let seam = find_min_energy_seam(&[], W, H, dummy_energy, &mut buffer);

        assert_eq!(seam, [3, 4, 3, 2]);
    }

    #[test]
    fn test_remove_min_energy_seam() {
        let mut img = vec![
            0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4,
            0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4,
            0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4,
            0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4
        ];

        let mut buffer = make_energy_buffer(W, H);

        remove_min_energy_seam(&mut img, W, H, dummy_energy, &mut buffer);

        img.truncate(3 * (W - 1) * H);

        // According to the dummy_energy function, the minimal energy seam
        // should be [2, 3, 4, 3] so we test if the "pixels" corresponding
        // to that seam are removed.
        assert_eq!(img, vec![
            0, 0, 0,   1, 1, 1,   3, 3, 3,   4, 4, 4,
            0, 0, 0,   1, 1, 1,   2, 2, 2,   4, 4, 4,
            0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,
            0, 0, 0,   1, 1, 1,   2, 2, 2,   4, 4, 4
        ]);
    }
}