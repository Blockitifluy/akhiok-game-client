//! Contains the `Entity`, `EntityType` and many critial entity types, such as: `GameType`.

use std::{fmt, str::FromStr};
use uuid::Uuid;

use crate::entities::{camera::CameraType, part_type::PartType};

/// The type of entity.
/// The enum stores a `Box` pointer to a struct
#[derive(Debug)]
pub enum EntityType {
    /// The base class for enums stores `nothing`.
    Base,
    /// The game entity.
    /// Used as a head of a EntityTree.
    Game(Box<GameType>),
    /// A building block entity.
    Part(Box<PartType>),
    Camera(Box<CameraType>),
}

/// An entity, used as a node in a tree hierarchry (`EntityTree`).
/// Used a container of `EntityType`
#[derive(Debug)]
pub struct Entity {
    /// The ID of the parent. Can be optional.
    pub parent_id: Option<Uuid>,
    /// A collection of IDs representing an entity's children.
    pub children_id: Vec<Uuid>,
    /// The non-unique name of the entity.
    name: String,
    /// The type of entity
    entity_type: EntityType,
    /// A unique identifier of the entity
    uuid: Uuid,
}
impl Entity {
    /// Creates a new entity, which is not parented to the anything or included inside the
    /// `EntityTree`
    /// # Note
    /// - For creation of entities, use `EntityTree.add_entity`.
    /// # Arguements
    /// - `name`: The name of the Entity
    /// - `entity_type`: The type of the Entity
    /// # Returns
    /// `Self`
    pub fn new(name: &str, entity_type: EntityType) -> Self {
        let name_string_ex = String::from_str(name);
        let Ok(name_str) = name_string_ex;
        Self {
            parent_id: None,
            children_id: vec![],
            name: name_str,
            entity_type,
            uuid: Uuid::new_v4(),
        }
    }

    /// Gets the current name of the Entity.
    /// # Returns
    /// The name of the entity.
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    /// Gets the read-only non-unique identifer of the Entity.
    /// # Returns
    /// The `Uuid` of the entity.
    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    /// Sets the name of the Entity
    /// # Arguements
    /// - `name`: the new name to be assigned to the node.
    pub fn set_name(&mut self, name: &str) {
        let name_string_ex = String::from_str(name);
        let Ok(name_str) = name_string_ex;
        self.name = name_str;
    }

    /// Gets the `EntityType` of the entity.
    pub fn get_type(&self) -> &EntityType {
        &self.entity_type
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: String::from_str("entity").unwrap(),
            entity_type: EntityType::Base,
            uuid: Uuid::new_v4(),
            children_id: vec![],
            parent_id: None,
        }
    }
}
impl fmt::Display for Entity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

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
