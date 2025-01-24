// src/visual/drawing.rs
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn draw_background(buffer: &mut Vec<u32>, width: usize, height: usize) {
    let dark_grey = 0x202020;
    for y in 0..height {
        for x in 0..width {
            buffer[y * width + x] = dark_grey;
        }
    }
}

pub fn draw_perceptron(buffer: &mut Vec<u32>, x_offset: usize, y_offset: usize, width: usize, height: usize, window_width: usize, window_height: usize) {
    let radius = 20;
    let input_positions = vec![
        (x_offset + 50, y_offset + height / 4),
        (x_offset + 50, y_offset + 3 * height / 4),
    ];
    let output_position = (x_offset + width - 50, y_offset + height / 2);

    // Draw connections
    for &(x, y) in &input_positions {
        draw_line(buffer, x, y, output_position.0, output_position.1, 0xFFFFFF, window_width, window_height);
    }

    // Draw input nodes
    for &(x, y) in &input_positions {
        draw_circle(buffer, x, y, radius, 0x0000FF, window_width, window_height);
    }

    // Draw output node
    draw_circle(buffer, output_position.0, output_position.1, radius, 0x0000FF, window_width, window_height);
}

pub fn draw_circle(buffer: &mut Vec<u32>, cx: usize, cy: usize, radius: usize, color: u32, width: usize, height: usize) {
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

pub fn draw_line(buffer: &mut Vec<u32>, x0: usize, y0: usize, x1: usize, y1: usize, color: u32, width: usize, height: usize) {
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

pub fn draw_border(buffer: &mut Vec<u32>, width: usize, height: usize) {
    let border_color = 0xFFFFFF;
    let border_thickness = 5;

    // Top and bottom borders
    for y in 0..border_thickness {
        for x in 0..width {
            buffer[y * width + x] = border_color;
            buffer[(height - 1 - y) * width + x] = border_color;
        }
    }

    // Left and right borders
    for x in 0..border_thickness {
        for y in 0..height {
            buffer[y * width + x] = border_color;
            buffer[y * width + (width - 1 - x)] = border_color;
        }
    }
}