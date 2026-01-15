//! Contains the `Entity`, `EntityType` and many critial entity types, such as: `GameType`.

use std::fmt;
use uuid::Uuid;

use crate::entities::types::{
    camera_type::Camera, game_type::Game, io_service::InputService, part_type::Part,
};

// TODO: Wrap EntityType's items with Box<>
trait_enum::trait_enum! {
/// The type of entity
#[derive(Debug)]
pub enum EntityType: EntityTrait {
    /// The base class for enums stores `nothing`.
    Base,
    /// The game entity.
    /// Used as a head of a EntityTree.
    Game,
    /// A building block entity.
    Part,
    /// A camera used for rendering
    Camera,
    /// A service providing Input and Output support
    InputService,
}
}

/// A trait that every entity should use.
pub trait EntityTrait {
    /// Gets called every frame.
    /// # Arguements
    /// - `delta`: the time between the last to second to last frame
    fn update(&mut self, _delta: f32) {}
}

/// The base entity: has no propetries or unique methods.
#[derive(Debug)]
pub struct Base;
impl EntityTrait for Base {}

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
    entity_type: Box<EntityType>,
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
    pub fn new(name: &str, entity_type: Box<EntityType>) -> Self {
        let name_str = name.to_string();
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
        let name_str = name.to_string();
        self.name = name_str;
    }

    /// Gets the `EntityType` of the entity.
    pub fn get_type(&self) -> &EntityType {
        &self.entity_type
    }

    /// Gets the `EntityType` of the entity as a mutable reference.
    pub fn get_type_mut(&mut self) -> &mut EntityType {
        &mut self.entity_type
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            name: "entity".to_string(),
            entity_type: Box::new(EntityType::Base(Base)),
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
