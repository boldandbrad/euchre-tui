use crate::engine::game::Game;

// interface callback repr
pub enum InterfaceCallback {
    None,
    Exit,
    QuitToSplash,
    SetupNewGame,
    StartGame { game: Game },
}
