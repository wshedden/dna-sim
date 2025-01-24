// src/main.rs
#![allow(dead_code)]
#![allow(unused_variables)]

use minifb::{Key, Window, WindowOptions};
use rand::Rng;
mod visual;

const INITIAL_WIDTH: usize = 1200;
const INITIAL_HEIGHT: usize = 800;

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
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let perceptron = Perceptron::new(2);
    let mut buffer: Vec<u32> = vec![0; INITIAL_WIDTH * INITIAL_HEIGHT];

    let mut panel_manager = visual::panel_manager::PanelManager::new(10);
    panel_manager.add_panel(visual::panel::Panel::with_draw_fn(50, 50, 400, 200, Box::new(visual::drawing::draw_perceptron)));
    panel_manager.add_panel(visual::panel::Panel::with_draw_fn(500, 50, 400, 200, Box::new(visual::drawing::draw_perceptron)));
    panel_manager.add_panel(visual::panel::Panel::with_draw_fn(50, 300, 1100, 400, Box::new(visual::drawing::draw_perceptron)));
    panel_manager.add_panel(visual::panel::Panel::with_draw_fn(950, 50, 200, 700, Box::new(visual::drawing::draw_perceptron)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (width, height) = window.get_size();
        buffer.resize(width * height, 0);

        visual::drawing::draw_background(&mut buffer, width, height);
        panel_manager.update_positions(width, height);
        panel_manager.draw(&mut buffer, width, height);
        visual::drawing::draw_border(&mut buffer, width, height);
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}