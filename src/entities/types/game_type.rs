//! Contains the `GameType` entity variant

use crate::entities::entity::EntityTrait;

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
pub struct Game {
    /// The game genre
    pub genre: GameGenre,
}
impl EntityTrait for Game {}
