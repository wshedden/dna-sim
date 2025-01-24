// src/visual/panel.rs
#![allow(dead_code)]
#![allow(unused_variables)]

pub struct Panel {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    draw_fn: Option<Box<dyn Fn(&mut Vec<u32>, usize, usize, usize, usize, usize, usize)>>,
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
        draw_fn: Box<dyn Fn(&mut Vec<u32>, usize, usize, usize, usize, usize, usize)>,
    ) -> Self {
        Panel { x, y, width, height, draw_fn: Some(draw_fn) }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, window_width: usize, window_height: usize) {
        // Draw panel background
        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                if x < window_width && y < window_height {
                    buffer[y * window_width + x] = 0x202020;
                }
            }
        }

        // Draw panel border
        let border_color = 0xFFFFFF;
        for x in self.x..self.x + self.width {
            if x < window_width {
                if self.y < window_height {
                    buffer[self.y * window_width + x] = border_color;
                }
                if self.y + self.height - 1 < window_height {
                    buffer[(self.y + self.height - 1) * window_width + x] = border_color;
                }
            }
        }
        for y in self.y..self.y + self.height {
            if y < window_height {
                if self.x < window_width {
                    buffer[y * window_width + self.x] = border_color;
                }
                if self.x + self.width - 1 < window_width {
                    buffer[y * window_width + (self.x + self.width - 1)] = border_color;
                }
            }
        }

        // Draw custom content
        if let Some(draw_fn) = &self.draw_fn {
            draw_fn(buffer, self.x, self.y, self.width, self.height, window_width, window_height);
        }
    }

    // Getter and setter methods
    pub fn x(&self) -> usize {
        self.x
    }

    pub fn set_x(&mut self, x: usize) {
        self.x = x;
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn set_y(&mut self, y: usize) {
        self.y = y;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}