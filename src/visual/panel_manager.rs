// src/visual/panel_manager.rs
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::visual::panel::Panel;

pub struct PanelManager {
    panels: Vec<Panel>,
    padding: usize,
}

impl PanelManager {
    pub fn new(padding: usize) -> Self {
        PanelManager {
            panels: Vec::new(),
            padding,
        }
    }

    pub fn add_panel(&mut self, panel: Panel) {
        self.panels.push(panel);
    }

    pub fn update_positions(&mut self, screen_width: usize, screen_height: usize) {
        let mut x_offset = self.padding;
        let mut y_offset = self.padding;

        for panel in &mut self.panels {
            if x_offset + panel.width() + self.padding > screen_width {
                x_offset = self.padding;
                y_offset += panel.height() + self.padding;
            }

            if y_offset + panel.height() + self.padding > screen_height {
                break; // No more space for panels
            }

            panel.set_x(x_offset);
            panel.set_y(y_offset);

            x_offset += panel.width() + self.padding;
        }

        // Ensure no collisions
        let mut changes = Vec::new();
        for i in 0..self.panels.len() {
            for j in (i + 1)..self.panels.len() {
                if self.is_colliding(&self.panels[i], &self.panels[j]) {
                    changes.push((i, j));
                }
            }
        }

        for (i, j) in changes {
            self.resolve_collision(i, j, screen_width, screen_height);
        }
    }

    fn is_colliding(&self, panel1: &Panel, panel2: &Panel) -> bool {
        let x1 = panel1.x();
        let y1 = panel1.y();
        let w1 = panel1.width();
        let h1 = panel1.height();

        let x2 = panel2.x();
        let y2 = panel2.y();
        let w2 = panel2.width();
        let h2 = panel2.height();

        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }

    fn resolve_collision(&mut self, i: usize, j: usize, screen_width: usize, screen_height: usize) {
        let (panel1, panel2) = self.panels.split_at_mut(j);
        let panel1 = &mut panel1[i];
        let panel2 = &mut panel2[0];

        if panel1.x() + panel1.width() + self.padding <= screen_width {
            panel2.set_x(panel1.x() + panel1.width() + self.padding);
        } else if panel1.y() + panel1.height() + self.padding <= screen_height {
            panel2.set_y(panel1.y() + panel1.height() + self.padding);
        } else {
            panel2.set_x(self.padding);
            panel2.set_y(panel1.y() + panel1.height() + self.padding);
        }
    }

    pub fn draw(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for panel in &self.panels {
            panel.draw(buffer, width, height);
        }
    }
}