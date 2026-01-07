#[derive(PartialEq)]
pub enum GameState {
    MainMenu,
    Settings,
    Scoreboard,
    PlayingSurvival,
    PlayingTimer
}

pub enum GameEndAction {
    Restart,
    MainMenu
}
