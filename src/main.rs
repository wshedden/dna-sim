#![allow(dead_code)]
#![allow(unused_variables)]

use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

struct Perceptron {
    weights: Vec<f32>,
}

impl Perceptron {
    fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights = (0..n).map(|_| rng.gen_range(-1.0..1.0)).collect();
        Perceptron { weights }
    }

    fn predict(&self, inputs: &[f32]) -> f32 {
        let sum: f32 = self.weights.iter().zip(inputs).map(|(w, i)| w * i).sum();
        if sum >= 0.0 { 1.0 } else { -1.0 }
    }
}

fn draw_perceptron(buffer: &mut Vec<u32>, perceptron: &Perceptron) {
    let radius = 20;
    let input_positions = vec![(100, 200), (100, 400)];
    let output_position = (700, 300);

    // Draw input nodes
    for &(x, y) in &input_positions {
        draw_circle(buffer, x, y, radius, 0x0000FF);
    }

    // Draw output node
    draw_circle(buffer, output_position.0, output_position.1, radius, 0x0000FF);

    // Draw connections
    for &(x, y) in &input_positions {
        draw_line(buffer, x, y, output_position.0, output_position.1, 0xFFFFFF);
    }
}

fn draw_circle(buffer: &mut Vec<u32>, cx: usize, cy: usize, radius: usize, color: u32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let dx = x as isize - cx as isize;
            let dy = y as isize - cy as isize;
            if dx * dx + dy * dy <= (radius * radius) as isize {
                buffer[y * WIDTH + x] = color;
            }
        }
    }
}

fn draw_line(buffer: &mut Vec<u32>, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let dx = (x1 as isize - x0 as isize).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 as isize - y0 as isize).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut x0 = x0 as isize;
    let mut y0 = y0 as isize;

    loop {
        if x0 >= 0 && x0 < WIDTH as isize && y0 >= 0 && y0 < HEIGHT as isize {
            buffer[y0 as usize * WIDTH + x0 as usize] = color;
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

fn main() {
    let mut window = Window::new(
        "Perceptron Visualization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let perceptron = Perceptron::new(2);
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_perceptron(&mut buffer, &perceptron);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}