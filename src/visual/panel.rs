// src/visual/panel.rs
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{WIDTH, HEIGHT};

pub struct Panel {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    draw_fn: Option<Box<dyn Fn(&mut Vec<u32>, usize, usize, usize, usize)>>,
}

impl Panel {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Panel { x, y, width, height, draw_fn: None }
    }

    pub fn with_draw_fn(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        draw_fn: Box<dyn Fn(&mut Vec<u32>, usize, usize, usize, usize)>,
    ) -> Self {
        Panel { x, y, width, height, draw_fn: Some(draw_fn) }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>) {
        // Draw panel background
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                if x < WIDTH && y < HEIGHT {
                    buffer[y * WIDTH + x] = 0x202020;
                }
            }
        }

        // Draw panel border
        let border_color = 0xFFFFFF;
        for x in self.x..self.x + self.width {
            if x < WIDTH {
                if self.y < HEIGHT {
                    buffer[self.y * WIDTH + x] = border_color;
                }
                if self.y + self.height - 1 < HEIGHT {
                    buffer[(self.y + self.height - 1) * WIDTH + x] = border_color;
                }
            }
        }
        for y in self.y..self.y + self.height {
            if y < HEIGHT {
                if self.x < WIDTH {
                    buffer[y * WIDTH + self.x] = border_color;
                }
                if self.x + self.width - 1 < WIDTH {
                    buffer[y * WIDTH + (self.x + self.width - 1)] = border_color;
                }
            }
        }

        // Draw custom content
        if let Some(draw_fn) = &self.draw_fn {
            draw_fn(buffer, self.x, self.y, self.width, self.height);
        }
    }
}