struct Energy {
    value: u32,
    backptr: usize
}

fn find_min_energy_seam<F>(img: &[&[u8]], w: usize, h: usize, energy_function: F) -> Vec<usize>
    where F: Fn(&[&[u8]], usize, usize, usize, usize) -> u32
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

fn remove_min_energy_seam<F>(img: &[&[u8]], w: usize, h: usize, energy_function: F) -> Vec<u8>
    where F: Fn(&[&[u8]], usize, usize, usize, usize) -> u32
{
    let seam = find_min_energy_seam(img, w, h, energy_function);

    let new_w = w - 1;
    let mut new_img = vec![0; 3 * new_w * h];

    for y in 0..h {
        let x = seam[h - 1 - y];

        // location of the start of the row, in new_img
        let row_start = 3 * y * new_w;
        // end location of the part before the seam, in new_img
        let end_first = row_start + 3 * x;
        // end location of the part after the seam, in new_img
        let end = row_start + 3 * new_w;

        assert!(row_start <= end_first);
        assert!(end_first <= end);

        new_img[row_start .. end_first].copy_from_slice(&img[y][.. 3 * x]);
        new_img[end_first .. end].copy_from_slice(&img[y][3 * x + 3 ..]);
    }

    new_img
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

    fn dummy_energy(_img: &[&[u8]], _w: usize, _h: usize, x: usize, y: usize) -> u32 {
        ENERGIES[y][x]
    }

    #[test]
    fn test_find_min_energy_seam() {
        let seam = find_min_energy_seam(&[&[]], W, H, dummy_energy);

        assert_eq!(seam, [3, 4, 3, 2]);
    }

    #[test]
    fn test_remove_min_energy_seam() {
        let img: &[&[u8]] = &[
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4],
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4],
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4],
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3,   4, 4, 4]
        ];

        let new_img = remove_min_energy_seam(img, W, H, dummy_energy);
        let new_img_view: Vec<&[u8]> = new_img.chunks_exact(3 * (W - 1)).collect();

        assert_eq!(new_img.len(), 3 * (W - 1) * H);

        // According to the dummy_energy function, the minimal energy seam
        // should be [2, 3, 4, 3] so we test if the "pixels" corresponding
        // with that seam are removed.
        assert_eq!(new_img_view, &[
            &[0, 0, 0,   1, 1, 1,   3, 3, 3,   4, 4, 4],
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   4, 4, 4],
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   3, 3, 3],
            &[0, 0, 0,   1, 1, 1,   2, 2, 2,   4, 4, 4]
        ]);
    }
}