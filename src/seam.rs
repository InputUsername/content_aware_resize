use copy_in_place::copy_in_place;

struct Energy {
    value: u32,
    backptr: usize
}

pub fn find_min_energy_seam<F>(img: &[u8], w: usize, h: usize, energy_function: F) -> Vec<usize>
    where F: Fn(&[u8], usize, usize, usize, usize) -> u32
{
    let mut energy_buf: Vec<Energy> = Vec::with_capacity(w * h);
    for x in 0..w {
        energy_buf.push(Energy {
            value: energy_function(img, w, h, x, 0),
            backptr: 0
        });
    }

    for y in 1..h {
        for x in 0..w {
            let prev_row_start = (y - 1) * w;
            let lo = x.saturating_sub(1);
            let hi = usize::min(x + 1, w - 1) + 1;

            let (prev_min, prev_min_x) = energy_buf[prev_row_start + lo .. prev_row_start + hi].iter()
                .zip(lo..hi)
                .min_by_key(|(e, _)| e.value)
                .unwrap();
            let prev_min_value = prev_min.value;

            energy_buf.push(Energy {
                value: prev_min_value + energy_function(img, w, h, x, y),
                backptr: prev_min_x
            });
        }
    }

    let mut seam = Vec::with_capacity(h);

    let last_row_start = (h - 1)*w;
    let (mut seam_x_pos, _) = energy_buf[last_row_start..].iter()
        .enumerate()
        .min_by_key(|(_, e)| e.value)
        .unwrap();

    for y in (0..h).rev() {
        seam.push(seam_x_pos);
        seam_x_pos = energy_buf[y * w + seam_x_pos].backptr;
    }

    seam
}

pub fn remove_min_energy_seam<F>(img: &mut [u8], w: usize, h: usize, energy_function: F)
    where F: Fn(&[u8], usize, usize, usize, usize) -> u32
{
    let seam = find_min_energy_seam(img, w, h, energy_function);

    for y in 0..h {
        let row_start = 3 * y * (w - 1);
        let x_idx = row_start + 3 * seam[h - 1 - y];

        assert!(x_idx <= 3 * w * h);

        // move all pixels after the seam one position to the left
        copy_in_place(img, x_idx + 3 .. 3 * w * h, x_idx);
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
        let seam = find_min_energy_seam(&[], W, H, dummy_energy);

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

        remove_min_energy_seam(&mut img, W, H, dummy_energy);

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