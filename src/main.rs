use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use chrono::Local;

mod ui;
mod file;
mod audio;
mod grid;
mod arrow;
mod functions;
mod gamestate;

use ui::skin_loader::*;
use file::*;
use audio::Audio;
use grid::*;
use arrow::*;
use functions::*;
use gamestate::*;

#[macroquad::main("Arrow Puzzle")]
async fn main() {
    const GRID_SIZE: usize = 6;
    const CELL_SIZE: f32 = 80.0;
    const NAV_BAR_HEIGHT: f32 = 70.0;
    const MENU_BUTTON_WIDTH: f32 = 200.0;
    const MENU_BUTTON_HEIGHT: f32 = 40.0;

    let settings: SettingsFile = match read_json("settings.json") {
        Ok(f) => f,
        Err(_) => { // Provide default settings
            SettingsFile {
                player_name: "".to_string(),
                timer_mode_duration: 30.0,
                sound_fx: false
            }
        }
    };
    let mut audio: Audio = Audio::new(settings.sound_fx).expect("Failed to initialize audio");
    let mut sound_fx_input: bool = audio.sound_fx;

    let mut game_state = GameState::MainMenu;

    let mut scoreboard: SaveFile = SaveFile { games_saved: Vec::new() };
    let mut grid = generate_grid(GRID_SIZE);
    let mut score: i32 = 0;
    let mut health: i32 = 0;
    let mut timer: f32 = 1.0;
    let mut timer_mode_duration: f32 = settings.timer_mode_duration;
    let mut timer_input_buffer = timer_mode_duration.to_string();
    let font_size = 32.0;
    let mut first_row: usize = 0;

    let mut player_name = settings.player_name.clone();
    let mut player_name_input = settings.player_name.clone();

    let skin_loader = SkinLoader::new();
    let skin = skin_loader.await.get_skin();
    root_ui().push_skin(&skin);

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let dt = get_frame_time();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let offset = grid_offset(GRID_SIZE, CELL_SIZE, screen_w, screen_h, NAV_BAR_HEIGHT);

        let button_x = (screen_w - MENU_BUTTON_WIDTH) / 2.0;
        let table_x = screen_w * 0.1;
        let table_y = screen_h * 0.1;
        let table_width = screen_w - ((screen_w * 0.1) * 2.0);
        let table_height = screen_h - ((screen_h * 0.1) * 2.0);

        match game_state {
            GameState::MainMenu => {
                // Main Menu Window
                if widgets::Button::new("Survival").position(vec2(button_x, screen_h * 0.2)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    grid = generate_grid(GRID_SIZE);
                    score = 0;
                    health = 3;
                    timer = 0.0001;
                    
                    audio.play_button();
                    game_state = GameState::PlayingSurvival;
                }

                if widgets::Button::new("Timer").position(vec2(button_x, screen_h * 0.3)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    grid = generate_grid(GRID_SIZE);
                    score = 0;
                    health = 1;
                    timer = timer_mode_duration;

                    audio.play_button();
                    game_state = GameState::PlayingTimer;
                }

                if widgets::Button::new("Scoreboard").position(vec2(button_x, screen_h * 0.4)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    first_row = 0;
                    scoreboard = match read_json("scoreboard.json") {
                        Ok(f) => f,
                        Err(_) => { SaveFile { games_saved: Vec::new() } }
                    };
                    
                    audio.play_button();
                    game_state = GameState::Scoreboard;
                }

                if widgets::Button::new("Settings").position(vec2(button_x, screen_h * 0.5)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    audio.play_button();
                    game_state = GameState::Settings;
                }

                if widgets::Button::new("Quit").position(vec2(button_x, screen_h * 0.6)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    std::process::exit(0);
                }
            }

            GameState::Scoreboard => {
                // Scoreboard Window
                draw_scrollable_table(table_x, table_y, table_width, table_height, &scoreboard.games_saved, &mut first_row);

                if widgets::Button::new("Back").position(vec2(button_x, screen_h * 0.8)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    audio.play_button();
                    game_state = GameState::MainMenu;
                }
            }

            GameState::Settings => {
                // Settings Window
                draw_text("Player Name", (screen_w / 2.0) - 80.0, (screen_h * 0.1) + font_size / 2.5, font_size, WHITE);

                widgets::InputText::new(hash!("player_name_input"))
                    .position(vec2(screen_w / 2.0 - 100.0, (screen_h * 0.1) + font_size))
                    .size(vec2(200.0, 32.0))
                    .ui(&mut root_ui(), &mut player_name_input);

                draw_text("Timer Duration (min: 5 sec.)", (screen_w / 2.0) - 180.0, (screen_h * 0.3) + font_size / 2.5, font_size, WHITE);

                widgets::InputText::new(hash!("timer_input"))
                    .position(vec2(screen_w / 2.0 - 100.0, screen_h * 0.3))
                    .size(vec2(200.0, 32.0))
                    .ui(&mut root_ui(), &mut timer_input_buffer);

                if widgets::Button::new(format!("Sound FX - {}", if sound_fx_input { "ON" } else { "OFF" })).position(vec2(button_x, screen_h * 0.5)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    sound_fx_input = !sound_fx_input;
                    audio.play_button();
                }

                if widgets::Button::new("Apply").position(vec2(button_x, screen_h * 0.7)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    if let Ok(value) = timer_input_buffer.parse::<f32>() {
                        timer_mode_duration = if value < 5.0 { 5.0 } else { value }
                    }

                    timer_mode_duration = (timer_mode_duration / 5.0).round() * 5.0;
                    timer_input_buffer = format!("{:.0}", timer_mode_duration);

                    audio.sound_fx = sound_fx_input;
                    player_name = player_name_input.clone();

                    match write_json("settings.json", &SettingsFile {
                        player_name: player_name_input.clone(),
                        timer_mode_duration: timer_mode_duration,
                        sound_fx: audio.sound_fx}) {
                            Ok(_) => { println!("Settings saved") },
                            Err(e) => { println!("{:?}", e) }
                    };
                    
                    audio.play_button();
                    game_state = GameState::MainMenu;
                }

                if widgets::Button::new("Back").position(vec2(button_x, screen_h * 0.8)).size(vec2(MENU_BUTTON_WIDTH, MENU_BUTTON_HEIGHT)).ui(&mut root_ui()) {
                    timer_input_buffer = format!("{:.0}", timer_mode_duration);
                    sound_fx_input = audio.sound_fx;
                    player_name_input = player_name.clone();
                    
                    audio.play_button();
                    game_state = GameState::MainMenu;
                }
            }

            GameState::PlayingSurvival | GameState::PlayingTimer => {
                // Gameplay Window
                if !(health <= 0 || timer <= 0.0) {
                    if grid_is_empty(&grid) {
                        if draw_regenerate_button(screen_w, screen_h) == Some(true) {
                            grid = generate_grid(GRID_SIZE);
                        }
                    } else {
                        if let Some((x, y)) = cell_from_mouse(GRID_SIZE, CELL_SIZE, offset) {
                            if can_remove(&grid, x, y, GRID_SIZE) {
                                grid[y][x] = None;
                                score += 1;
                            } else if game_state == GameState::PlayingTimer {
                                audio.play_mistake();
                            } else {
                                health -= 1;
                                audio.play_mistake();
                            }
                        }
                    }
                    
                    if game_state == GameState::PlayingTimer {
                        timer -= dt;
                    } else if game_state == GameState::PlayingSurvival {
                        timer += dt;
                    }

                    if health <= 0 || timer <= 0.0 {
                        audio.play_success();
                    }
                }
                
                draw_arrow_grid(&grid, GRID_SIZE, CELL_SIZE, offset);
                draw_nav_bar(score, health, timer, screen_w, NAV_BAR_HEIGHT, &mut game_state, &mut audio);

                if health <= 0 || timer <= 0.0 {
                    if let Some(action) = draw_game_end_screen(screen_w, screen_h, score) {
                        let gamemode: String;
                        let time: f32;

                        match game_state {
                            GameState::PlayingSurvival => {
                                gamemode = "Survival".to_string();
                                time = timer; // Stopwatch
                            }

                            GameState::PlayingTimer => {
                                gamemode = "Timer".to_string();
                                time = timer_mode_duration; // Timer duration
                            }

                            _ => {
                                gamemode = "Unknown".to_string();
                                time = 0.0;
                            }
                        };
                        
                        match append_to_scoreboard("scoreboard.json", SaveData {
                            player_name: player_name.clone(),
                            gamemode,
                            time,
                            score,
                            datetime: Local::now().format("%d-%m-%Y %H:%M:%S").to_string() }) {
                                Ok(_) => println!("Score saved"),
                                Err(e) => println!("{:?}", e)
                        }

                        match action {
                            GameEndAction::Restart => {
                                grid = generate_grid(GRID_SIZE);
                                score = 0;
                                health = 3;
                                timer = if game_state == GameState::PlayingTimer {
                                    timer_mode_duration
                                } else {
                                    0.0001
                                };

                                audio.play_button();
                            }

                            GameEndAction::MainMenu => {
                                audio.play_button();
                                game_state = GameState::MainMenu;
                            }
                        }
                    }
                }
            }
        }

        next_frame().await;
    }
}