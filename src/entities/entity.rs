use std::{fmt, str::FromStr};

use crate::mesh::Mesh;

#[derive(Debug)]
pub enum EntityType {
    Base,
    Game(Box<GameType>),
    Part(Box<PartType>),
}

pub struct Entity<'children> {
    name: String,
    entity_type: EntityType,
    children: Vec<&'children Entity<'children>>,
}
impl<'children> Entity<'children> {
    pub fn new(name: &str, entity_type: EntityType) -> Self {
        let name_string_ex = String::from_str(name);
        let Ok(name_str) = name_string_ex;
        Self {
            name: name_str,
            entity_type,
            children: vec![],
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn set_name(&mut self, name: &str) {
        let name_string_ex = String::from_str(name);
        let Ok(name_str) = name_string_ex;
        self.name = name_str;
    }

    pub fn get_type(&self) -> &EntityType {
        &self.entity_type
    }

    pub fn get_mut_type(&mut self) -> &mut EntityType {
        &mut self.entity_type
    }
}

impl<'children> Default for Entity<'children> {
    fn default() -> Self {
        Self {
            name: String::from_str("entity").unwrap(),
            entity_type: EntityType::Base,
            children: vec![],
        }
    }
}
impl<'children> fmt::Display for Entity<'children> {
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
