use crate::engine::game::Game;

pub enum InterfaceCallback {
    None,
    Exit,
    QuitToSplash,
    SetupNewGame,
    StartGame { game: Game },
}
