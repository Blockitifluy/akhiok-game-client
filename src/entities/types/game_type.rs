//! Contains the `GameType` entity variant

use crate::entities::entity::EntityTrait;

/// The game's genre
#[derive(Debug, Default)]
pub enum GameGenre {
    /// Action
    Action,
    /// Adventure
    Adventure,
    /// None
    #[default]
    Undefined,
}

/// The game entity type.
/// Used as a head of a `EntityTree`.
#[derive(Debug)]
pub struct Game {
    /// The game genre
    pub genre: GameGenre,
}
impl Game {
    /// Creates a new Game entity.
    /// # Arguements
    /// - `genre`: the genre of the game
    /// # Return
    /// `Game` entity type
    pub fn new(genre: GameGenre) -> Self {
        Self { genre }
    }
}

impl EntityTrait for Game {}

impl Default for Game {
    fn default() -> Self {
        Self {
            genre: GameGenre::Undefined,
        }
    }
}
