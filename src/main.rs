// src/main.rs
#![allow(dead_code)]
#![allow(unused_variables)]

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
mod visual;

const WIDTH: usize = 1000;
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

    let panel1 = visual::panel::Panel::with_draw_fn(50, 50, 400, 200, Box::new(visual::drawing::draw_perceptron));
    let panel2 = visual::panel::Panel::with_draw_fn(500, 50, 400, 200, Box::new(visual::drawing::draw_perceptron));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        visual::drawing::draw_background(&mut buffer);
        panel1.draw(&mut buffer);
        panel2.draw(&mut buffer);
        visual::drawing::draw_border(&mut buffer);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}