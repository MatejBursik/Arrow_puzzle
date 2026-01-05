use macroquad::prelude::*;

use crate::grid::*;

pub fn draw_arrow(x: f32, y: f32, arrow: &Arrow, cell_size: f32) {
    let center = vec2(x + cell_size / 2.0, y + cell_size / 2.0);

    let size = 20.0;

    match arrow.dir {
        Direction::Right => {
            draw_triangle(
                center + vec2(-size, -size),
                center + vec2(-size, size),
                center + vec2(size, 0.0),
                arrow.color
            );
        }

        Direction::Left => {
            draw_triangle(
                center + vec2(size, -size),
                center + vec2(size, size),
                center + vec2(-size, 0.0),
                arrow.color
            );
        }

        Direction::Up => {
            draw_triangle(
                center + vec2(-size, size),
                center + vec2(size, size),
                center + vec2(0.0, -size),
                arrow.color
            );
        }

        Direction::Down => {
            draw_triangle(
                center + vec2(-size, -size),
                center + vec2(size, -size),
                center + vec2(0.0, size),
                arrow.color
            );
        }
    }
}

pub fn draw_arrow_grid(grid: &Grid, grid_size: usize, cell_size: f32, offset: Vec2) {
    for y in 0..grid_size {
        for x in 0..grid_size {
            let px = offset.x + x as f32 * cell_size;
            let py = offset.y + y as f32 * cell_size;

            // draw_rectangle_lines(px, py, cell_size, cell_size, 2.0, BLACK);

            if let Some(arrow) = &grid[y][x] {
                draw_arrow(px, py, arrow, cell_size);
            }
        }
    }
}
