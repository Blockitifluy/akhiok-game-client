//! Contains the `EntityTree` struct used for the entity heirarchry.

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use uuid::Uuid;

use crate::entities::{
    camera::CameraType,
    entity::{Entity, EntityType, GameGenre, GameType},
};

/// A tree of entities.
/// Queries by a `HashMap` and `Uuid`s.
#[derive(Debug, Default)]
pub struct EntityTree {
    /// The identitier of the head (usually `GameType`).
    /// Can be `None`.
    pub head: Option<Uuid>,
    pub main_camera: Option<Uuid>,
    /// The indentifier for every part.
    pub parts: Vec<Uuid>,
    entity_map: HashMap<Uuid, Rc<RefCell<Entity>>>,
}
impl EntityTree {
    /// Creates a new entity.
    /// # Arguements
    /// - `name`: The name of the entity
    /// - `entity_type`: The `EntityType` of the entity
    /// # Returns
    /// A reference counted RefCell of the `Entity`.
    pub fn add_entity(&mut self, name: &str, entity_type: EntityType) -> Rc<RefCell<Entity>> {
        let entity = Rc::new(RefCell::new(Entity::new(name, entity_type)));
        let id = entity.borrow().get_uuid();
        self.entity_map.insert(id, entity.clone());
        if let EntityType::Part(_) = entity.borrow().get_type() {
            self.parts.push(id);
        }
        entity
    }

    /// Creates a new entity, that is initally parented to another entity.
    /// # Arguements
    /// - `name`: The name of the entity
    /// - `entity_type`: The `EntityType` of the entity
    /// - `parent`: A mutable reference of the entity
    /// # Returns
    /// A result where it could be either:
    /// - A reference counted RefCell of the `Entity`.
    /// - An error message
    pub fn add_entity_with_parent(
        &mut self,
        name: &str,
        entity_type: EntityType,
        parent: &mut Entity,
    ) -> Result<Rc<RefCell<Entity>>, &'static str> {
        let entity = self.add_entity(name, entity_type);
        let mut entity_borrow = entity.borrow_mut();
        self.set_parent(entity_borrow.deref_mut(), Some(parent))?;
        Ok(entity.clone())
    }

    /// Adds a new head of the `Game` entity type.
    /// # Returns
    /// A reference counted RefCell of the `Entity`.
    pub fn add_head(&mut self) -> Rc<RefCell<Entity>> {
        let head = Rc::new(RefCell::new(Entity::new(
            "Game",
            EntityType::Game(Box::new(GameType {
                genre: GameGenre::Action,
            })),
        )));
        let head_borrow = head.borrow();
        let id = head_borrow.get_uuid();
        self.head = Some(id);
        self.entity_map.insert(id, head.clone());
        head.clone()
    }

    /// Gets the head of the entity type.
    /// # Returns
    /// An option of a reference counted RefCell of the `Entity`.
    pub fn get_head(&self) -> Option<Rc<RefCell<Entity>>> {
        let head_id = self.head?;

        Some(self.entity_map[&head_id].clone())
    }

    pub fn add_main_camera(
        &mut self,
        parent: Option<&mut Entity>,
        camera_type: CameraType,
    ) -> Option<Rc<RefCell<Entity>>> {
        let camera_type_box = Box::new(camera_type);

        let camera = Rc::new(RefCell::new(Entity::new(
            "Camera",
            EntityType::Camera(camera_type_box),
        )));
        let mut camera_borrow = camera.borrow_mut();
        if let Err(err) = self.set_parent(camera_borrow.deref_mut(), parent) {
            println!("couldn't parent camera: {}", err);
            return None;
        }

        let id = camera_borrow.get_uuid();

        self.main_camera = Some(id);
        self.entity_map.insert(id, camera.clone());
        Some(camera.clone())
    }

    pub fn get_main_camera(&self) -> Option<Rc<RefCell<Entity>>> {
        let camera_id = self.main_camera?;

        Some(self.entity_map[&camera_id].clone())
    }

    // SUGGESTION: get_entity and it's variants should return a result when borrowing is
    // unsuccessful
    // SUGGESTION: get_entity_refcell

    /// Gets an entity based on the `id`.
    /// # Arguements
    /// - `id`: The unique indentifier of the entity
    /// # Returns
    /// An option to a reference to an entity
    pub fn get_entity(&self, id: Uuid) -> Option<Ref<Entity>> {
        let entity_null = self.entity_map.get(&id);
        if let Some(entity) = entity_null {
            return Some(entity.borrow());
        }
        None
    }

    /// Gets an entity (as an mutable reference) based on the `id`.
    /// # Arguements
    /// - `id`: The unique indentifier of the entity
    /// # Returns
    /// An option to a mutable reference to an entity
    pub fn get_entity_mut(&self, id: Uuid) -> Option<RefMut<Entity>> {
        let entity_null = self.entity_map.get(&id);
        if let Some(entity) = entity_null {
            return Some(entity.borrow_mut());
        }
        None
    }

    // Parent

    /// Gets an entity's parent.
    /// # Arguements
    /// - `entity`: a borrow of an entity
    /// # Returns
    /// An option to a reference of an entity
    pub fn get_parent(&self, entity: &Entity) -> Option<Ref<Entity>> {
        let id = entity.parent_id?;

        let relative_null = self.entity_map.get(&id);

        if let Some(relative) = relative_null {
            return Some(relative.borrow());
        }
        None
    }

    /// Gets an entity's parent as a mutable reference.
    /// # Arguements
    /// - `entity`: a borrow of an entity
    /// # Returns
    /// An option to a reference of an entity
    pub fn get_parent_mut(&mut self, entity: &Entity) -> Option<RefMut<Entity>> {
        let id = entity.parent_id?;

        let relative_null = self.entity_map.get_mut(&id);

        if let Some(relative) = relative_null {
            let borrow_attempt = relative.try_borrow_mut();
            if let Ok(borrow) = borrow_attempt {
                return Some(borrow);
            }
            println!("cannot borrow parent ID: {}", id);
        }
        None
    }

    /// Sets the parent to an entity. Can be unsuccessful.
    /// # Arguements
    /// - `entity`: An mutable reference to an entity
    /// - `parent`: A entity used as `entity`'s new parent
    /// # Returns
    /// An error message if a parent was unsuccessful.
    pub fn set_parent(
        &mut self,
        mut entity: &mut Entity,
        parent: Option<&mut Entity>,
    ) -> Result<(), &'static str> {
        let self_id = entity.get_uuid();

        let Some(new_parent) = parent else {
            entity.parent_id = None;
            if let Some(mut former_parent) = self.get_parent_mut(entity.deref()) {
                let index = former_parent
                    .children_id
                    .iter()
                    .position(|x| *x == self_id)
                    .unwrap();
                former_parent.children_id.remove(index);
            }
            return Ok(());
        };

        if self_id == new_parent.get_uuid() {
            return Err("can't parent to self");
        }

        for desc_id in self.get_descendents_id(entity.deref()) {
            if desc_id == self_id {
                return Err("can't parent to descendent");
            }
        }

        let new_id = new_parent.get_uuid();
        let entity_mut = entity.deref_mut();
        if let Some(mut former_parent) = self.get_parent_mut(entity_mut) {
            let index = former_parent
                .children_id
                .iter()
                .position(|x| *x == self_id)
                .unwrap();
            former_parent.children_id.remove(index);
        }
        entity_mut.parent_id = Some(new_id);
        entity_mut.children_id.push(new_id);
        Ok(())
    }

    // Ancestors

    /// Gets an entity's ancestors.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of `uuid`s referencing an entity
    pub fn get_ancestors_id(&self, entity: &Entity) -> Vec<Uuid> {
        let mut parent;
        let mut current = entity;
        let mut ancestors = Vec::<Uuid>::with_capacity(16);

        while current.parent_id.is_some() {
            let parent_id_null = entity.parent_id;
            let Some(parent_id) = parent_id_null else {
                break;
            };

            parent = self.get_parent(entity).unwrap();
            current = &parent;
            ancestors.push(parent_id);
        }

        ancestors.shrink_to_fit();
        ancestors
    }

    /// Gets an entity's ancestors as mutable references.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of a mutable reference to an entity
    pub fn get_ancestors_mut(&self, entity: &Entity) -> Vec<RefMut<Entity>> {
        let ancestors_id = self.get_ancestors_id(entity);
        let mut ancestors = Vec::with_capacity(16);

        for id in ancestors_id {
            let entity = self.get_entity_mut(id).unwrap();
            ancestors.push(entity);
        }
        ancestors.shrink_to_fit();
        ancestors
    }

    /// Gets an entity's ancestors as immutable references.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of a immutable reference to an entity
    pub fn get_ancestors(&self, entity: &Entity) -> Vec<Ref<Entity>> {
        let ancestors_id = self.get_ancestors_id(entity);
        let mut ancestors = Vec::with_capacity(16);

        for id in ancestors_id {
            let entity = self.get_entity(id).unwrap();
            ancestors.push(entity);
        }
        ancestors.shrink_to_fit();
        ancestors
    }

    // Children

    /// Gets an entity's children.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of references to an entity
    pub fn get_children(&self, entity: &Entity) -> Vec<Ref<Entity>> {
        let mut children = Vec::with_capacity(16);

        for child_id in entity.children_id.clone() {
            children.push(self.entity_map[&child_id].borrow());
        }

        children.shrink_to_fit();
        children
    }

    // If I have to expierence this shit again I am rewriting the entire project in C++
    // "Don't worry bro you just need 5 bloated smart points that halt performance and look ugly as
    // shit"
    // Worse day of my life was writing 7 line of the definition of bullshit
    // This rant is longer that the FUCKING function
    // Javascript is better
    // Everything else in this project is being written in Go, C# or C++
    // To solve this problem we need to a 15 new different problems with the least helpful error
    // messages ever
    // I finally got this 14' monitor so I can see the fucking unhelpful error messages from the
    // borrow checker
    // Genuinly the first time I have gotten this angry
    // If this is the reason why I don't a job so be it
    // New contender on worst programming language?
    // Favourite thing in this project is not coding for days only design because I feel like Sir
    // Francis Drake the way I am circumnavgating the borrow checker
    // Great way to start of the year, Graydon Hoare
    // Fuck it reinventing the Von Newman Archietchure just to avoid this abombination of a
    // language

    /// Gets an entity's children as an mutable reference to an entity.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of mutable references to an entity
    pub fn get_children_mut(&mut self, entity: &Entity) -> Vec<RefMut<Entity>> {
        let mut children: Vec<RefMut<Entity>> = Vec::with_capacity(entity.children_id.len());

        for child_id in entity.children_id.clone() {
            let child_ref = self.get_entity_mut(child_id);
            if let Some(child) = child_ref {
                children.push(child);
            }
        }
        children
    }

    /// Gets an entity's descendent as identitiers.
    /// # Arguement
    /// - `entity`: A borrow of an entity
    /// # Retutrns
    /// A collection of IDs representing the entity's descendent.
    pub fn get_descendents_id(&self, entity: &Entity) -> Vec<Uuid> {
        let mut descendents = self.get_children(entity);
        let mut stack_rel: Vec<Uuid> = descendents.iter().map(|e| e.get_uuid()).collect();

        while !stack_rel.is_empty() {
            let rel_id_null = stack_rel.pop();
            let Some(rel_id) = rel_id_null else {
                break;
            };

            let ent = self.entity_map.get(&rel_id).unwrap().borrow();
            let mut children = ent.children_id.to_owned();
            stack_rel.append(&mut children);
            descendents.push(ent);
        }
        stack_rel
    }

    /// Gets an entity's descendents as a reference.
    /// # Arguement
    /// - `entity`: A borrow of an entity
    /// # Returns
    /// A collection of entities.
    pub fn get_descendents(&self, entity: &Entity) -> Vec<Ref<Entity>> {
        let descendents_id = self.get_descendents_id(entity);
        let mut descendents: Vec<Ref<Entity>> = Vec::with_capacity(descendents_id.len());

        for id in descendents_id {
            let descend_ref = self.get_entity(id);
            if let Some(decend) = descend_ref {
                descendents.push(decend);
            }
        }
        descendents
    }

    /// Getsa an entity's descendents as an mutable reference.
    /// # Arguements
    /// - `entity`: A borrow of an entity
    /// # Returns
    /// A collection of entities as mutable references.
    pub fn get_descendents_mut(&mut self, entity: &Entity) -> Vec<RefMut<Entity>> {
        let descendents_id = self.get_descendents_id(entity);
        let mut descendents: Vec<RefMut<Entity>> = Vec::with_capacity(descendents_id.len());

        for id in descendents_id {
            let descend_ref = self.get_entity_mut(id);
            if let Some(decend) = descend_ref {
                descendents.push(decend);
            }
        }
        descendents
    }
}
