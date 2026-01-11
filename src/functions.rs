use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets};

use crate::grid::*;
use crate::gamestate::*;
use crate::file::SaveData;

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
        None => return false
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

    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0,0.0,0.0,0.4));

    if widgets::Button::new("New Grid").position(vec2(x, y)).size(vec2(button_width, button_height)).ui(&mut root_ui()) {
        return Some(true);
    }

    None
}

pub fn draw_game_end_screen(screen_w: f32, screen_h: f32, score: i32) -> Option<GameEndAction> {
    let button_width = 220.0;
    let button_height = 50.0;
    let spacing = 20.0;
    let center_x = screen_w / 2.0;

    draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::new(0.0,0.0,0.0,0.6));

    // Score
    let score_text = format!("Final Score: {}", score);
    let score_size = 32.0;
    let score_dims = measure_text(&score_text, None, score_size as u16, 1.0);

    draw_text(&score_text, center_x - score_dims.width / 2.0, screen_h * 0.4, score_size, LIGHTGRAY);
    
    // Restart button
    let restart_x = center_x - button_width / 2.0;
    let restart_y = screen_h * 0.5;

    if widgets::Button::new("Restart").position(vec2(restart_x, restart_y)).size(vec2(button_width, button_height)).ui(&mut root_ui()) {
        return Some(GameEndAction::Restart);
    }

    // Main menu button
    let menu_y = restart_y + button_height + spacing;

    if widgets::Button::new("Main Menu").position(vec2(restart_x, menu_y)).size(vec2(button_width, button_height)).ui(&mut root_ui()) {
        return Some(GameEndAction::MainMenu);
    }

    None
}

fn format_time(seconds: f32) -> String {
    let secs = seconds.max(0.0) as i32;
    let minutes = secs / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}", minutes, seconds)
}

pub fn draw_nav_bar(score: i32, health :i32, timer: f32, screen_w: f32, nav_bar_height: f32, game_state: &mut GameState) {
    let font_size = 32.0;
    let button_width = 90.0;
    let button_height = 32.0;
    let mut center_text_color = WHITE;
    
    draw_rectangle(0.0, 0.0, screen_w, nav_bar_height, BLACK);

    // Score (left)
    let score_text = format!("Score: {}", score);
    
    draw_text(&score_text, 20.0, nav_bar_height / 2.0 + font_size / 2.5, font_size, WHITE);

    // Timer / Health (center)
    let center_text: String;

    if *game_state == GameState::PlayingSurvival {
        center_text = format!("Health: {}", health);
        if health <= 1 {
            center_text_color = RED;
        }
    } else if *game_state == GameState::PlayingTimer {
        if timer < 6.0 {
            center_text_color = RED;
        }
        center_text = format!("Timer: {}", format_time(timer));
    } else {
        center_text = "".to_string();
    }

    draw_text(&center_text, (screen_w / 2.0) - button_width, nav_bar_height / 2.0 + font_size / 2.5, font_size, center_text_color);

    // Back button (right)
    if !(health <= 0 || timer <= 0.0) {
        if widgets::Button::new("Back").position(vec2(screen_w - button_width - 20.0, nav_bar_height / 2.0 - button_height / 2.0)).size(vec2(button_width, button_height)).ui(&mut root_ui()) {
            *game_state = GameState::MainMenu;
        }
    }
}

pub fn draw_scrollable_table(x: f32, y: f32, width: f32, height: f32, rows: &[SaveData], first_row: &mut usize) {
    const ROW_HEIGHT: f32 = 26.0;
    const HEADER_HEIGHT: f32 = 30.0;

    let (mx, my) = mouse_position();

    let visible_rows = ((height - HEADER_HEIGHT) / ROW_HEIGHT).floor() as usize;
    let max_first_row = rows.len().saturating_sub(visible_rows);

    if mx >= x && mx <= x + width && my >= y && my <= y + height {
        let (_, wheel) = mouse_wheel();

        if wheel != 0.0 {
            if wheel > 0.0 {
                *first_row = first_row.saturating_sub(1);
            } else {
                *first_row = (*first_row + 1).min(max_first_row);
            }
        }
    }

    // Header
    draw_rectangle(x, y, width, HEADER_HEIGHT, DARKBLUE);

    draw_text("ID", x + 10.0, y + 20.0, 20.0, WHITE);
    draw_text("Gamemode", x + 10.0 + (width * 0.1), y + 20.0, 20.0, WHITE);
    draw_text("Time", x + 10.0 + (width * 0.3), y + 20.0, 20.0, WHITE);
    draw_text("Score", x + 10.0 + (width * 0.5), y + 20.0, 20.0, WHITE);
    draw_text("Datetime", x + 10.0 + (width * 0.7), y + 20.0, 20.0, WHITE);

    // Draw visible rows only
    for i in 0..visible_rows {
        let row_index = *first_row + i;
        let row_y = y + HEADER_HEIGHT + i as f32 * ROW_HEIGHT;

        let hovered =
            mx >= x && mx <= x + width &&
            my >= row_y && my <= row_y + ROW_HEIGHT;

        draw_rectangle(x, row_y, width, ROW_HEIGHT, if hovered { DARKGRAY } else { GRAY });

        draw_text(&row_index.to_string(), x + 10.0, row_y + 18.0, 18.0, WHITE);
        draw_text(&rows[row_index].gamemode, x + 10.0 + (width * 0.1), row_y + 18.0, 18.0, WHITE);
        draw_text(&format!("{:.2}", &rows[row_index].time), x + 10.0 + (width * 0.3), row_y + 18.0, 18.0, WHITE);
        draw_text(&rows[row_index].score.to_string(), x + 10.0 + (width * 0.5), row_y + 18.0, 18.0, WHITE);
        draw_text(&rows[row_index].datetime, x + 10.0 + (width * 0.7), row_y + 18.0, 18.0, WHITE);
    }

    // Disable scissor
    set_default_camera();
}
