#[derive(PartialEq, Clone)]
pub enum GameState {
    MainMenu,
    Settings,
    Scoreboard,
    PlayingSurvival,
    PlayingTimer
}

#[derive(PartialEq)]
pub enum GameEndAction {
    Restart,
    MainMenu
}
