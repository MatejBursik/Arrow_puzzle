use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::grid::*;
use crate::gamestate::GameState;

pub fn cell_from_mouse(grid_size: usize, cell_size: f32, offset: Vec2) -> Option<(usize, usize)> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mx, my) = mouse_position();

    if mx < offset.x || my < offset.y {
        return None;
    }

    let x = ((mx - offset.x) / cell_size) as usize;
    let y = ((my - offset.y) / cell_size) as usize;

    if x < grid_size && y < grid_size {
        return Some((x, y));
    }
    
    None
}

pub fn can_remove(grid: &Grid, x: usize, y: usize, grid_size: usize) -> bool {
    let arrow = match &grid[y][x] {
        Some(a) => a,
        None => return false,
    };

    match arrow.dir {
        Direction::Right => {
            for nx in x+1..grid_size {
                if grid[y][nx].is_some() {
                    return false;
                }
            }
        }

        Direction::Left => {
            for nx in (0..x).rev() {
                if grid[y][nx].is_some() {
                    return false;
                }
            }
        }

        Direction::Down => {
            for ny in y+1..grid_size {
                if grid[ny][x].is_some() {
                    return false;
                }
            }
        }

        Direction::Up => {
            for ny in (0..y).rev() {
                if grid[ny][x].is_some() {
                    return false;
                }
            }
        }
    }

    true
}

pub fn draw_regenerate_button(screen_w: f32, screen_h: f32) -> Option<bool> {
    let button_width = 220.0;
    let button_height = 50.0;

    let x = (screen_w - button_width) / 2.0;
    let y = (screen_h - button_height) / 2.0;

    let mouse = vec2(mouse_position().0, mouse_position().1);
    let hovered =
        mouse.x >= x && mouse.x <= x + button_width &&
        mouse.y >= y && mouse.y <= y + button_height;

    let color = if hovered { LIGHTGRAY } else { GRAY };
    let text = "New Grid";
    let font_size = 30.0;
    let text_dims = measure_text(text, None, font_size as u16, 1.0);

    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0,0.0,0.0,0.4));
    draw_rectangle(x, y, button_width, button_height, color);
    draw_text(text, x + (button_width - text_dims.width) / 2.0, y + (button_height + text_dims.height) / 2.0 - 4.0, font_size, BLACK);

    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        return Some(true);
    }

    None
}

pub fn draw_nav_bar(points: u32, screen_w: f32, nav_bar_height: f32, game_state: &mut GameState) {
    draw_rectangle(0.0, 0.0, screen_w, nav_bar_height, BLACK);

    // Points (left)
    let points_text = format!("Points: {}", points);
    let font_size = 32.0;

    draw_text(&points_text, 20.0, nav_bar_height / 2.0 + font_size / 2.5, font_size, WHITE);

    // Back button (right)
    let button_width = 90.0;
    let button_height = 32.0;

    if widgets::Button::new("Back").position(vec2(screen_w - button_width - 20.0, nav_bar_height / 2.0 - button_height / 2.0)).size(vec2(button_width, button_height)).ui(&mut root_ui()) {
        *game_state = GameState::MainMenu;
    }
}
