use kira::{AudioManager, AudioManagerSettings};
use kira::sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings};
use std::error::Error;

pub struct Audio {
    manager: AudioManager,
    pub sound_fx: bool,
    pub music: bool,
    button_sound_fx: StaticSoundData,
    mistake_sound_fx: StaticSoundData,
    success_sound_fx: StaticSoundData,
    background_menu_music: StaticSoundData,
    background_gameplay_music: StaticSoundData,
    music_handle: Option<StaticSoundHandle>
}

impl Audio {
    pub fn new(sound_fx: bool, music: bool) -> Result<Self, Box<dyn Error>> {
        let manager = AudioManager::new(AudioManagerSettings::default())?;

        let button_sound_fx = StaticSoundData::from_file("assets/sound_fx/beep-313342.mp3")?;
        let mistake_sound_fx = StaticSoundData::from_file("assets/sound_fx/error-mistake-sound-effect-incorrect-answer-437420.mp3")?;
        let success_sound_fx = StaticSoundData::from_file("assets/sound_fx/tada-234709.mp3")?;

        let background_menu_music = StaticSoundData::from_file("assets/music/phatphrogstudio-cyber-attack-datastorm-rebellion-477469.mp3")?
            .with_settings(StaticSoundSettings::default().loop_region(..));
        let background_gameplay_music = StaticSoundData::from_file("assets/music/phatphrogstudio-internal-fury-furyx27s-dance-477470.mp3")?
            .with_settings(StaticSoundSettings::default().loop_region(..));

        Ok(Self {manager, sound_fx, music, button_sound_fx, mistake_sound_fx, success_sound_fx, background_menu_music, background_gameplay_music, music_handle: None})
    }

    pub fn play_button(&mut self) {
        if self.sound_fx {
            let _ = self.manager.play(self.button_sound_fx.clone());
        }
    }

    pub fn play_mistake(&mut self) {
        if self.sound_fx {
            let _ = self.manager.play(self.mistake_sound_fx.clone());
        }
    }

    pub fn play_success(&mut self) {
        if self.sound_fx {
            let _ = self.manager.play(self.success_sound_fx.clone());
        }
    }

    pub fn start_background_menu_music(&mut self) {
        if self.music {
            if self.music_handle.is_none() {
                if let Ok(handle) = self.manager.play(self.background_menu_music.clone()) {
                    self.music_handle = Some(handle);
                }
            }
        }
    }

    pub fn start_background_gameplay_music(&mut self) {
        if self.music {
            if self.music_handle.is_none() {
                if let Ok(handle) = self.manager.play(self.background_gameplay_music.clone()) {
                    self.music_handle = Some(handle);
                }
            }
        }
    }

    pub fn stop_music(&mut self) {
        if self.music {
            if let Some(mut handle) = self.music_handle.take() {
                let _ = handle.stop(Default::default());
            }
        }
    }
}
