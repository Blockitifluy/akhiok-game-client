use std::{fmt, rc::Rc, str::FromStr};

use uuid::Uuid;

use crate::mesh::Mesh;

#[derive(Debug)]
pub enum EntityType {
    Base,
    Game(Box<GameType>),
    Part(Box<PartType>),
}

pub struct Entity<'a> {
    name: String,
    entity_type: EntityType,
    parent: Option<Rc<Entity<'a>>>,
    children: Vec<&'a Entity<'a>>,
    uuid: Uuid,
}
impl<'a> Entity<'a> {
    pub fn new(name: &str, entity_type: EntityType, parent: Option<Rc<Entity<'a>>>) -> Self {
        let name_string_ex = String::from_str(name);
        let Ok(name_str) = name_string_ex;
        Self {
            name: name_str,
            entity_type,
            parent,
            uuid: Uuid::new_v4(),
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

    pub fn get_parent(&self) -> Option<Rc<Entity<'a>>> {
        if let Some(ref parent) = self.parent {
            return Some(Rc::clone(parent));
        }
        None
    }

    pub fn get_parent_mut(&mut self) -> Option<Rc<Entity<'a>>> {
        if let Some(ref mut parent) = self.parent {
            return Some(Rc::clone(parent));
        };
        None
    }

    pub fn get_descendents(&self) -> Vec<&'a Entity<'a>> {
        let mut descendents = Vec::<&'a Entity<'a>>::with_capacity(16);
        let mut stack = descendents.clone();

        while stack.len() > 0 {
            let desc_null = stack.pop();
            let Some(desc) = desc_null else {
                continue;
            };

            let mut children = desc.children.clone();

            descendents.append(&mut children);
            stack.append(&mut children);
        }

        descendents.shrink_to_fit();
        return descendents;
    }

    pub fn set_parent(&mut self, parent: Option<Rc<Entity<'a>>>) -> Result<(), &'static str> {
        // TODO: remove element from former parent and add to new parent
        let Some(new_parent) = parent else {
            self.parent = None;
            return Ok(());
        };

        if self.uuid == new_parent.uuid {
            return Err("Can't parent entity with it's self");
        }

        let descendents = self.get_descendents();
        for descend in descendents {
            if self.uuid != descend.uuid {
                continue;
            }
            return Err("cyclical hierachry detected");
        }

        self.parent = Some(new_parent);
        Ok(())
    }

    pub fn get_children(&self) -> &[&'a Entity<'a>] {
        self.children.as_slice()
    }
}

impl<'a> Default for Entity<'a> {
    fn default() -> Self {
        Self {
            name: String::from_str("entity").unwrap(),
            entity_type: EntityType::Base,
            parent: None,
            uuid: Uuid::new_v4(),
            children: vec![],
        }
    }
}
impl<'a> fmt::Display for Entity<'a> {
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
