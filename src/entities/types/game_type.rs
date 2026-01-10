//! Contains the `GameType` entity variant

/// The game's genre
#[derive(Debug)]
pub enum GameGenre {
    /// Action
    Action,
    /// Adventure
    Adventure,
}

/// The game entity type.
/// Used as a head of a `EntityTree`.
#[derive(Debug)]
pub struct GameType {
    /// The game genre
    pub genre: GameGenre,
}
