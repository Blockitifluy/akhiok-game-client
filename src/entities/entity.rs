use std::{fmt, str::FromStr};

use uuid::Uuid;

use crate::mesh::Mesh;

#[derive(Debug)]
pub enum EntityType {
    Base,
    Game(Box<GameType>),
    Part(Box<PartType>),
}

#[derive(Debug)]
pub struct Entity {
    pub parent_id: Option<Uuid>,
    pub children_id: Vec<Uuid>,
    name: String,
    entity_type: EntityType,
    uuid: Uuid,
}
impl Entity {
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

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn set_name(&mut self, name: &str) {
        let name_string_ex = String::from_str(name);
        let Ok(name_str) = name_string_ex;
        self.name = name_str;
    }

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

#[derive(Debug)]
pub enum GameGenre {
    Action,
    Adventure,
}

#[derive(Debug)]
pub struct GameType {
    pub genre: GameGenre,
}

#[derive(Debug)]
pub struct PartType {
    pub mesh: Mesh,
}
