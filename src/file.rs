use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Write, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct SettingsFile {
    pub player_name: String,
    pub timer_mode_duration: f32,
    pub sound_fx: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub player_name: String,
    pub gamemode: String,
    pub time: f32,
    pub score: i32,
    pub datetime: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveFile {
    pub games_saved: Vec<SaveData>
}

pub fn write_json<T: Serialize>(file_name: &str, data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    let mut file = File::create(file_name)?;

    file.write_all(json.as_bytes())?;
    
    Ok(())
}

pub fn read_json<T: for<'de> Deserialize<'de>>(file_name: &str) -> Result<T> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;

    Ok(data)
}

pub fn append_to_scoreboard(file_name: &str, save_data: SaveData) -> Result<()> {
    let mut save_file: SaveFile = match read_json(file_name) {
        Ok(f) => f,
        Err(_) => SaveFile { games_saved: Vec::new() }
    };
    
    save_file.games_saved.push(save_data);
    write_json(file_name, &save_file)?;
    
    Ok(())
}