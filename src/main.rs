use macroquad::prelude::*;

mod grid;
mod arrow;
mod functions;

use grid::*;
use arrow::*;
use functions::*;

#[macroquad::main("Arrow Puzzle")]
async fn main() {
    const GRID_SIZE: usize = 6;
    const CELL_SIZE: f32 = 80.0;
    const NAV_BAR_HEIGHT: f32 = 70.0;

    let mut grid = generate_grid(GRID_SIZE);
    let mut points: u32 = 0;

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let screen_w = screen_width();
        let screen_h = screen_height();
        let offset = grid_offset(GRID_SIZE, CELL_SIZE, screen_w, screen_h, NAV_BAR_HEIGHT);

        if let Some((x, y)) = cell_from_mouse(GRID_SIZE, CELL_SIZE, offset) {
            if can_remove(&grid, x, y, GRID_SIZE) {
                grid[y][x] = None;
                points += 1;
            }
        }

        draw_nav_bar(points, screen_w, NAV_BAR_HEIGHT);
        draw_arrow_grid(&grid, GRID_SIZE, CELL_SIZE, offset);
        
        if grid_is_empty(&grid) {
            if draw_regenerate_button(screen_w, screen_h) == Some(true) {
                grid = generate_grid(GRID_SIZE);
            }
        }

        next_frame().await;
    }
}