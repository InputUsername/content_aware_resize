pub fn basic(img: &[&[u8]], w: usize, h: usize, x: usize, y: usize) -> u32 {
    let h1 = x.saturating_sub(1);
    let h2 = (x + 1).min(w - 1);
    let v1 = y.saturating_sub(1);
    let v2 = (y + 1).min(h - 1);

    let ph1 = &img[y][3*h1..3*h1+3];
    let ph2 = &img[y][3*h2..3*h2+3];
    let pv1 = &img[v1][3*x..3*x+3];
    let pv2 = &img[v2][3*x..3*x+3];

    let mut dx = 0;
    let mut dy = 0;

    for i in 0..3 {
        let ha = ph1[i].max(ph2[i]);
        let hb = ph1[i].min(ph2[i]);
        let va = pv1[i].max(pv2[i]);
        let vb = pv1[i].min(pv2[i]);

        let dh = (ha - hb) as u32;
        let dv = (va - vb) as u32;

        dx += dh * dh;
        dy += dv * dv;
    }

    return dx + dy;
}