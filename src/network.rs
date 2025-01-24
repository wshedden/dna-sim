// src/network.rs
#![allow(dead_code)]
#![allow(unused_variables)]

use rand::Rng;

pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Vec<Vec<f32>>>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let mut weights = Vec::new();

        for i in 0..layers.len() - 1 {
            let layer_weights: Vec<Vec<f32>> = (0..layers[i + 1])
                .map(|_| (0..layers[i]).map(|_| rng.gen_range(-1.0..1.0)).collect())
                .collect();
            weights.push(layer_weights);
        }

        Network { layers, weights }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, x_offset: usize, y_offset: usize, width: usize, height: usize, window_width: usize, window_height: usize) {
        let radius = 20;
        let layer_spacing = width / (self.layers.len() - 1);
        let mut positions = Vec::new();

        for (i, &layer_size) in self.layers.iter().enumerate() {
            let node_spacing = height / (layer_size + 1);
            let layer_positions: Vec<(usize, usize)> = (0..layer_size)
                .map(|j| (x_offset + i * layer_spacing, y_offset + (j + 1) * node_spacing))
                .collect();
            positions.push(layer_positions);
        }

        for (i, layer) in positions.iter().enumerate().skip(1) {
            for (j, &(x1, y1)) in layer.iter().enumerate() {
                for (k, &(x0, y0)) in positions[i - 1].iter().enumerate() {
                    draw_line(buffer, x0, y0, x1, y1, 0xFFFFFF, window_width, window_height);
                }
            }
        }

        for layer in &positions {
            for &(x, y) in layer {
                draw_circle(buffer, x, y, radius, 0x0000FF, window_width, window_height);
            }
        }
    }
}

fn draw_circle(buffer: &mut Vec<u32>, cx: usize, cy: usize, radius: usize, color: u32, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            let dx = x as isize - cx as isize;
            let dy = y as isize - cy as isize;
            if dx * dx + dy * dy <= (radius * radius) as isize {
                buffer[y * width + x] = color;
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<u32>, x0: usize, y0: usize, x1: usize, y1: usize, color: u32, width: usize, height: usize) {
    let dx = (x1 as isize - x0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 as isize - y0 as isize).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;

    loop {
        if x0 >= 0 && x0 < width as isize && y0 >= 0 && y0 < height as isize {
            buffer[y0 as usize * width + x0 as usize] = color;
        }
        if x0 == x1 as isize && y0 == y1 as isize { break; }
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