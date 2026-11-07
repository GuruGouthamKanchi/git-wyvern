use crate::palette::Rgba;

/// Fast, deterministic 2D pixel canvas for retro pixel art rendering.
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Rgba>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Rgba::TRANSPARENT; width * height],
        }
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Rgba) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let idx = y as usize * self.width + x as usize;
            if color.3 == 255 {
                self.pixels[idx] = color;
            } else if color.3 > 0 {
                // Alpha blend
                let bg = self.pixels[idx];
                let a = color.3 as u32;
                if bg.3 == 0 {
                    // When painting onto a transparent background, avoid creating low-alpha translucent
                    // fringes that quantize to harsh black specks in 1-bit GIF transparency.
                    if a >= 160 {
                        self.pixels[idx] = Rgba(color.0, color.1, color.2, 255);
                    }
                } else {
                    let inv_a = 255 - a;
                    let r = ((color.0 as u32 * a + bg.0 as u32 * inv_a) / 255) as u8;
                    let g = ((color.1 as u32 * a + bg.1 as u32 * inv_a) / 255) as u8;
                    let b = ((color.2 as u32 * a + bg.2 as u32 * inv_a) / 255) as u8;
                    let out_a = if bg.3 == 255 {
                        255
                    } else {
                        (a + (bg.3 as u32 * inv_a) / 255).min(255) as u8
                    };
                    self.pixels[idx] = Rgba(r, g, b, out_a);
                }
            }
        }
    }

    #[inline]
    pub fn get_pixel(&self, x: i32, y: i32) -> Rgba {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.pixels[y as usize * self.width + x as usize]
        } else {
            Rgba::TRANSPARENT
        }
    }

    /// Bresenham's line algorithm with thickness
    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Rgba, width: i32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let radius = (width - 1) / 2;

        loop {
            if width <= 1 {
                self.set_pixel(x0, y0, color);
            } else {
                for rx in -radius..=radius {
                    for ry in -radius..=radius {
                        if rx * rx + ry * ry <= radius * radius + 1 {
                            self.set_pixel(x0 + rx, y0 + ry, color);
                        }
                    }
                }
            }

            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Scanline polygon filler for arbitrary convex/concave polygons
    pub fn fill_polygon(&mut self, pts: &[(i32, i32)], color: Rgba) {
        if pts.len() < 3 {
            return;
        }

        let min_y = pts.iter().map(|p| p.1).min().unwrap().max(0);
        let max_y = pts.iter().map(|p| p.1).max().unwrap().min(self.height as i32 - 1);

        for y in min_y..=max_y {
            let mut node_x = Vec::new();
            let mut j = pts.len() - 1;
            for i in 0..pts.len() {
                let p_i = pts[i];
                let p_j = pts[j];
                if (p_i.1 < y && p_j.1 >= y) || (p_j.1 < y && p_i.1 >= y) {
                    let num = (y - p_i.1) as f32 * (p_j.0 - p_i.0) as f32;
                    let den = (p_j.1 - p_i.1) as f32;
                    let x = p_i.0 as f32 + num / den;
                    node_x.push(x.round() as i32);
                }
                j = i;
            }

            node_x.sort_unstable();

            let mut k = 0;
            while k + 1 < node_x.len() {
                let start_x = node_x[k].max(0);
                let end_x = node_x[k + 1].min(self.width as i32 - 1);
                for x in start_x..=end_x {
                    self.set_pixel(x, y, color);
                }
                k += 2;
            }
        }
    }

    /// Scanline polygon filler with checkerboard Bayer-style dither pattern between two colors
    pub fn fill_polygon_dither(&mut self, pts: &[(i32, i32)], col1: Rgba, col2: Rgba) {
        if pts.len() < 3 {
            return;
        }

        let min_y = pts.iter().map(|p| p.1).min().unwrap().max(0);
        let max_y = pts.iter().map(|p| p.1).max().unwrap().min(self.height as i32 - 1);

        for y in min_y..=max_y {
            let mut node_x = Vec::new();
            let mut j = pts.len() - 1;
            for i in 0..pts.len() {
                let p_i = pts[i];
                let p_j = pts[j];
                if (p_i.1 < y && p_j.1 >= y) || (p_j.1 < y && p_i.1 >= y) {
                    let num = (y - p_i.1) as f32 * (p_j.0 - p_i.0) as f32;
                    let den = (p_j.1 - p_i.1) as f32;
                    let x = p_i.0 as f32 + num / den;
                    node_x.push(x.round() as i32);
                }
                j = i;
            }

            node_x.sort_unstable();

            let mut k = 0;
            while k + 1 < node_x.len() {
                let start_x = node_x[k].max(0);
                let end_x = node_x[k + 1].min(self.width as i32 - 1);
                for x in start_x..=end_x {
                    let c = if (x + y) % 2 == 0 { col1 } else { col2 };
                    self.set_pixel(x, y, c);
                }
                k += 2;
            }
        }
    }

    /// Fast scanline ellipse filler for contact drop shadows
    pub fn fill_ellipse(&mut self, cx: i32, cy: i32, rx: i32, ry: i32, color: Rgba) {
        if rx <= 0 || ry <= 0 {
            return;
        }
        let min_y = (cy - ry).max(0);
        let max_y = (cy + ry).min(self.height as i32 - 1);
        let rx_f = rx as f32;
        let ry_f = ry as f32;

        for y in min_y..=max_y {
            let dy = (y - cy) as f32 / ry_f;
            let term = 1.0 - dy * dy;
            if term >= 0.0 {
                let dx = (term.sqrt() * rx_f).round() as i32;
                let min_x = (cx - dx).max(0);
                let max_x = (cx + dx).min(self.width as i32 - 1);
                for x in min_x..=max_x {
                    self.set_pixel(x, y, color);
                }
            }
        }
    }

    /// Fast scanline circle filler
    pub fn fill_circle(&mut self, cx: i32, cy: i32, r: i32, color: Rgba) {
        self.fill_ellipse(cx, cy, r, r, color);
    }

    /// Stippled polygon filler (50% checkerboard matrix) for translucent retro GIF shadows
    pub fn fill_polygon_stipple_50(&mut self, pts: &[(i32, i32)], color: Rgba) {
        if pts.len() < 3 {
            return;
        }
        let min_y = pts.iter().map(|p| p.1).min().unwrap().max(0);
        let max_y = pts.iter().map(|p| p.1).max().unwrap().min(self.height as i32 - 1);

        for y in min_y..=max_y {
            let mut node_x = Vec::new();
            let mut j = pts.len() - 1;
            for i in 0..pts.len() {
                let p_i = pts[i];
                let p_j = pts[j];
                if (p_i.1 < y && p_j.1 >= y) || (p_j.1 < y && p_i.1 >= y) {
                    let num = (y - p_i.1) as f32 * (p_j.0 - p_i.0) as f32;
                    let den = (p_j.1 - p_i.1) as f32;
                    let x = p_i.0 as f32 + num / den;
                    node_x.push(x.round() as i32);
                }
                j = i;
            }

            node_x.sort_unstable();

            let mut k = 0;
            while k + 1 < node_x.len() {
                let start_x = node_x[k].max(0);
                let end_x = node_x[k + 1].min(self.width as i32 - 1);
                for x in start_x..=end_x {
                    if (x + y) % 2 == 0 {
                        self.set_pixel(x, y, color);
                    }
                }
                k += 2;
            }
        }
    }

    /// Stippled polygon filler (25% sparse matrix) for soft optical penumbra fringe
    pub fn fill_polygon_stipple_25(&mut self, pts: &[(i32, i32)], color: Rgba) {
        if pts.len() < 3 {
            return;
        }
        let min_y = pts.iter().map(|p| p.1).min().unwrap().max(0);
        let max_y = pts.iter().map(|p| p.1).max().unwrap().min(self.height as i32 - 1);

        for y in min_y..=max_y {
            let mut node_x = Vec::new();
            let mut j = pts.len() - 1;
            for i in 0..pts.len() {
                let p_i = pts[i];
                let p_j = pts[j];
                if (p_i.1 < y && p_j.1 >= y) || (p_j.1 < y && p_i.1 >= y) {
                    let num = (y - p_i.1) as f32 * (p_j.0 - p_i.0) as f32;
                    let den = (p_j.1 - p_i.1) as f32;
                    let x = p_i.0 as f32 + num / den;
                    node_x.push(x.round() as i32);
                }
                j = i;
            }

            node_x.sort_unstable();

            let mut k = 0;
            while k + 1 < node_x.len() {
                let start_x = node_x[k].max(0);
                let end_x = node_x[k + 1].min(self.width as i32 - 1);
                for x in start_x..=end_x {
                    if (x % 2 == 0) && (y % 2 == 0) {
                        self.set_pixel(x, y, color);
                    }
                }
                k += 2;
            }
        }
    }

    /// Stippled ellipse filler (50% checkerboard matrix) for soft shadow contacts
    pub fn fill_ellipse_stipple_50(&mut self, cx: i32, cy: i32, rx: i32, ry: i32, color: Rgba) {
        if rx <= 0 || ry <= 0 {
            return;
        }
        let min_y = (cy - ry).max(0);
        let max_y = (cy + ry).min(self.height as i32 - 1);
        let rx_f = rx as f32;
        let ry_f = ry as f32;

        for y in min_y..=max_y {
            let dy = (y - cy) as f32 / ry_f;
            let term = 1.0 - dy * dy;
            if term >= 0.0 {
                let dx = (term.sqrt() * rx_f).round() as i32;
                let min_x = (cx - dx).max(0);
                let max_x = (cx + dx).min(self.width as i32 - 1);
                for x in min_x..=max_x {
                    if (x + y) % 2 == 0 {
                        self.set_pixel(x, y, color);
                    }
                }
            }
        }
    }

    /// Stippled line drawer (50% checkerboard matrix)
    pub fn draw_line_stipple_50(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Rgba, width: i32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        let radius = (width - 1) / 2;

        loop {
            if width <= 1 {
                if (x0 + y0) % 2 == 0 {
                    self.set_pixel(x0, y0, color);
                }
            } else {
                for rx in -radius..=radius {
                    for ry in -radius..=radius {
                        if rx * rx + ry * ry <= radius * radius + 1 {
                            let px = x0 + rx;
                            let py = y0 + ry;
                            if (px + py) % 2 == 0 {
                                self.set_pixel(px, py, color);
                            }
                        }
                    }
                }
            }

            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Upscale 2x with exact nearest-neighbor integer clustering (2x2 pixels per logical pixel),
    /// replicating git-koi's exact 512x512 visual density.
    pub fn upscale_2x(&self) -> Canvas {
        let new_w = self.width * 2;
        let new_h = self.height * 2;
        let mut up = Canvas::new(new_w, new_h);

        for y in 0..self.height {
            for x in 0..self.width {
                let col = self.pixels[y * self.width + x];
                if col.3 > 0 {
                    let out_x = (x * 2) as i32;
                    let out_y = (y * 2) as i32;
                    up.set_pixel(out_x, out_y, col);
                    up.set_pixel(out_x + 1, out_y, col);
                    up.set_pixel(out_x, out_y + 1, col);
                    up.set_pixel(out_x + 1, out_y + 1, col);
                }
            }
        }
        up
    }

    /// Export raw RGBA byte buffer
    pub fn to_rgba_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.width * self.height * 4);
        for p in &self.pixels {
            bytes.push(p.0);
            bytes.push(p.1);
            bytes.push(p.2);
            bytes.push(p.3);
        }
        bytes
    }
}
