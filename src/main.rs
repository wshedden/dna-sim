// src/main.rs
#![allow(dead_code)]
#![allow(unused_variables)]

use minifb::{Key, Window, WindowOptions};
use std::rc::Rc;
mod visual;
mod network;

const INITIAL_WIDTH: usize = 1200;
const INITIAL_HEIGHT: usize = 800;

fn main() {
    let mut window = Window::new(
        "Network Visualization",
        INITIAL_WIDTH,
        INITIAL_HEIGHT,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let network = Rc::new(network::Network::new(vec![2, 3, 3, 1]));
    let mut buffer: Vec<u32> = vec![0; INITIAL_WIDTH * INITIAL_HEIGHT];

    let mut panel_manager = visual::panel_manager::PanelManager::new(30, 20);
    let network_clone = Rc::clone(&network);
    panel_manager.add_panel(visual::panel::Panel::with_draw_fn(50, 50, 400, 200, Box::new(move |buffer, x, y, w, h, ww, wh| {
        network_clone.draw(buffer, x, y, w, h, ww, wh, 20);
    })));
    let network_clone = Rc::clone(&network);
    panel_manager.add_panel(visual::panel::Panel::with_draw_fn(500, 50, 400, 200, Box::new(move |buffer, x, y, w, h, ww, wh| {
        network_clone.draw(buffer, x, y, w, h, ww, wh, 20);
    })));

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