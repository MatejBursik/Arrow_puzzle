use kira::{AudioManager, AudioManagerSettings, sound::static_sound::StaticSoundData};
use std::error::Error;

pub struct Audio {
    manager: AudioManager,
    button_sound_fx: StaticSoundData,
    mistake_sound_fx: StaticSoundData,
    success_sound_fx: StaticSoundData
}

impl Audio {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let manager = AudioManager::new(AudioManagerSettings::default())?;

        let button_sound_fx = StaticSoundData::from_file("assets/sound_fx/beep-313342.mp3")?;
        let mistake_sound_fx = StaticSoundData::from_file("assets/sound_fx/error-mistake-sound-effect-incorrect-answer-437420.mp3")?;
        let success_sound_fx = StaticSoundData::from_file("assets/sound_fx/tada-234709.mp3")?;

        Ok(Self {manager, button_sound_fx, mistake_sound_fx, success_sound_fx})
    }

    pub fn play_button(&mut self) {
        let _ = self.manager.play(self.button_sound_fx.clone());
    }

    pub fn play_mistake(&mut self) {
        let _ = self.manager.play(self.mistake_sound_fx.clone());
    }

    pub fn play_success(&mut self) {
        let _ = self.manager.play(self.success_sound_fx.clone());
    }
}
