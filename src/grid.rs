use rand::seq::SliceRandom;
use macroquad::{color::Color, math::Vec2, prelude::vec2};

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
pub struct Arrow {
    pub dir: Direction,
    pub color: Color
}

pub type Grid = Vec<Vec<Option<Arrow>>>;

fn path_is_clear(grid: &Grid, x: usize, y: usize, dir: &Direction, grid_size: usize) -> bool {
    match dir {
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

pub fn generate_grid(grid_size: usize) -> Grid {
    let mut grid = vec![vec![None; grid_size]; grid_size];
    let mut rng = rand::rng();

    // All positions shuffled
    let mut positions = Vec::new();

    for y in 0..grid_size {
        for x in 0..grid_size {
            positions.push((x, y));
        }
    }

    positions.shuffle(&mut rng);

    for (x, y) in positions {
        let mut dirs = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right
        ];

        let mut colors = vec![
            Color::new(0.0, 1.0, 0.0, 1.0), // Neon Green
            Color::new(1.0, 0.0, 1.0, 1.0), // Neon Magenta
            Color::new(0.0, 1.0, 1.0, 1.0), // Neon Cyan
            Color::new(1.0, 1.0, 0.0, 1.0), // Neon Yellow
            Color::new(1.0, 0.5, 0.0, 1.0), // Neon Orange
            Color::new(0.5, 0.0, 1.0, 1.0), // Neon Purple
            Color::new(1.0, 0.0, 0.5, 1.0), // Neon Pink
            Color::new(0.0, 0.5, 1.0, 1.0)  // Neon Blue
        ];

        dirs.shuffle(&mut rng);
        colors.shuffle(&mut rng);

        for dir in dirs {
            if path_is_clear(&grid, x, y, &dir, grid_size) {
                grid[y][x] = Some(Arrow { dir: dir, color: colors[0] });
                break;
            }
        }
    }

    grid
}

pub fn grid_is_empty(grid: &Grid) -> bool {
    grid.iter().all(|row| row.iter().all(|cell| cell.is_none()))
}

pub fn grid_offset(grid_size: usize, cell_size: f32, screen_width: f32, screen_height: f32, nav_bar_height: f32) -> Vec2 {
    let grid_px = grid_size as f32 * cell_size;

    let x = (screen_width - grid_px) / 2.0;
    let y = nav_bar_height + (screen_height - nav_bar_height - grid_px) / 2.0;

    vec2(x.max(0.0), y.max(nav_bar_height))
}
