//! Contains the `EntityTree` struct used for the entity heirarchry.

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use uuid::Uuid;

use crate::entities::{
    entity::{Entity, EntityType},
    types::{
        camera_type::Camera,
        game_type::{Game, GameGenre},
    },
};

// TODO: Add Child, Descendent and Ancestor iterators

/// A tree of entities.
/// Queries by a `HashMap` and `Uuid`s.
#[derive(Debug, Default)]
pub struct EntityTree {
    /// The identitier of the head (usually `GameType`).
    /// Can be `None`.
    pub head: Option<Uuid>,
    /// The identitier of the main camera.
    /// Can be `None`.
    pub main_camera: Option<Uuid>,
    /// The indentifier for every part.
    pub parts: Vec<Uuid>,
    /// A hashmap of all entity as values and their ID's as keys
    /// # Note
    /// Not to be edited directly use the provided methods instead.
    pub entity_map: HashMap<Uuid, Rc<RefCell<Entity>>>,
}
impl EntityTree {
    /// Creates a new entity.
    /// # Arguements
    /// - `name`: The name of the entity
    /// - `entity_type`: The `EntityType` of the entity
    /// # Returns
    /// A reference counted RefCell of the `Entity`.
    pub fn add_entity(&mut self, name: &str, entity_type: EntityType) -> Rc<RefCell<Entity>> {
        let entity = Rc::new(RefCell::new(Entity::new(name, Box::new(entity_type))));
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
            Box::new(EntityType::Game(Game {
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

    /// Adds a new main camera of the `Camera` entity type.
    /// # Arguements
    /// - `parent`: the camera's parent
    /// - `camera_type`: the camera_type variant
    /// # Returns
    /// An option of a reference counted RefCell of the camera `Entity`
    pub fn add_main_camera(
        &mut self,
        parent: Option<&mut Entity>,
        camera_type: Camera,
    ) -> Option<Rc<RefCell<Entity>>> {
        let camera = Rc::new(RefCell::new(Entity::new(
            "Camera",
            Box::new(EntityType::Camera(camera_type)),
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

    /// Gets the main camera
    /// # Returns
    /// An option of reference counted RefCell of the camera `Entity`
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
        let entity = self.entity_map.get(&id)?;
        Some(entity.borrow())
    }

    /// Gets an entity (as an mutable reference) based on the `id`.
    /// # Arguements
    /// - `id`: The unique indentifier of the entity
    /// # Returns
    /// An option to a mutable reference to an entity
    pub fn get_entity_mut(&self, id: Uuid) -> Option<RefMut<Entity>> {
        let entity = self.entity_map.get(&id)?;
        Some(entity.borrow_mut())
    }

    /// Gets an entity (as an reference counted ref cell) based on the `id`.
    /// # Arguements
    /// - `id`: The unique identitier of the entity
    /// # Returns
    /// An option of a reference counted ref cell to an entity.
    pub fn get_entity_rc(&self, id: Uuid) -> Option<Rc<RefCell<Entity>>> {
        let entity = self.entity_map.get(&id)?;
        Some(entity.clone())
    }

    /// Gets all entities inside of the tree.
    /// # Returns
    /// A collection of references to an entity
    pub fn get_entities(&self) -> Vec<Ref<Entity>> {
        self.entity_map.values().map(|e| e.borrow()).collect()
    }

    /// Gets all entities inside of the tree.
    /// # Returns
    /// A collection of mutable references to an entity
    pub fn get_entities_mut(&mut self) -> Vec<RefMut<Entity>> {
        self.entity_map.values().map(|e| e.borrow_mut()).collect()
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
    pub fn get_parent_mut(&self, entity: &Entity) -> Option<RefMut<Entity>> {
        let id = entity.parent_id?;

        let relative = self.entity_map.get(&id)?;

        let borrow_attempt = relative.try_borrow_mut();
        if let Ok(borrow) = borrow_attempt {
            return Some(borrow);
        }
        println!("cannot borrow parent ID: {}", id);
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
        new_parent.children_id.push(self_id);
        Ok(())
    }

    // Heirarchry Selection

    /// Finds the first child that has the name that is equal to `name`.
    /// # Arguements
    /// - `entity`: the entity
    /// - `name`: the name
    /// # Returns
    /// An optional reference entity
    pub fn find_first_child(&self, entity: &Entity, name: &str) -> Option<Ref<Entity>> {
        let entity = self
            .get_children(entity)
            .into_iter()
            .find(|e| e.get_name() == name)?;
        Some(entity)
    }

    /// Finds the first child that has the name that is equal to `name`.
    /// # Arguements
    /// - `entity`: the entity
    /// - `name`: the name
    /// # Returns
    /// An optional mutable reference entity
    pub fn find_first_child_mut(&self, entity: &Entity, name: &str) -> Option<RefMut<Entity>> {
        let entity = self
            .get_children_mut(entity)
            .into_iter()
            .find(|e| e.get_name() == name)?;
        Some(entity)
    }

    /// Finds the first descendent that has the name that is equal to `name`.
    /// # Arguements
    /// - `entity`: the entity
    /// - `name`: the name
    /// # Returns
    /// An optional reference entity
    pub fn find_first_descendent(&self, entity: &Entity, name: &str) -> Option<Ref<Entity>> {
        let entity = self
            .get_descendents(entity)
            .into_iter()
            .find(|e| e.get_name() == name)?;
        Some(entity)
    }

    /// Finds the first descendent that has the name that is equal to `name`.
    /// # Arguements
    /// - `entity`: the entity
    /// - `name`: the name
    /// # Returns
    /// An optional mutable reference entity
    pub fn find_first_descendent_mut(&self, entity: &Entity, name: &str) -> Option<RefMut<Entity>> {
        let entity = self
            .get_descendents_mut(entity)
            .into_iter()
            .find(|e| e.get_name() == name)?;
        Some(entity)
    }

    /// Finds the first ancestor descendent that has the name that is equal to `name`.
    /// # Arguements
    /// - `entity`: the entity
    /// - `name`: the name
    /// # Returns
    /// An optional mutable reference entity
    pub fn find_first_ancestor(&self, entity: &Entity, name: &str) -> Option<Ref<Entity>> {
        let entity = self
            .get_ancestors(entity)
            .into_iter()
            .find(|e| e.get_name() == name)?;
        Some(entity)
    }

    /// Finds the first ancestor that has the name that is equal to `name`.
    /// # Arguements
    /// - `entity`: the entity
    /// - `name`: the name
    /// # Returns
    /// An optional mutable reference entity
    pub fn find_first_ancestor_mut(&self, entity: &Entity, name: &str) -> Option<RefMut<Entity>> {
        let entity = self
            .get_ancestors_mut(entity)
            .into_iter()
            .find(|e| e.get_name() == name)?;
        Some(entity)
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
        self.get_ancestors_id(entity)
            .iter()
            .map(|id| self.entity_map[id].borrow_mut())
            .collect()
    }

    /// Gets an entity's ancestors as immutable references.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of a immutable reference to an entity
    pub fn get_ancestors(&self, entity: &Entity) -> Vec<Ref<Entity>> {
        self.get_ancestors_id(entity)
            .iter()
            .map(|id| self.entity_map[id].borrow())
            .collect()
    }

    // Children

    /// Gets an entity's children.
    /// # Arguements
    /// - `entity`: An entity
    /// # Returns
    /// A collection of references to an entity
    pub fn get_children(&self, entity: &Entity) -> Vec<Ref<Entity>> {
        entity
            .children_id
            .iter()
            .map(|id| self.entity_map[id].borrow())
            .collect()
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
    pub fn get_children_mut(&self, entity: &Entity) -> Vec<RefMut<Entity>> {
        entity
            .children_id
            .iter()
            .map(|id| self.entity_map[id].borrow_mut())
            .collect()
    }

    /// Gets an entity's descendent as identitiers.
    /// # Arguement
    /// - `entity`: A borrow of an entity
    /// # Retutrns
    /// A collection of IDs representing the entity's descendent.
    pub fn get_descendents_id(&self, entity: &Entity) -> Vec<Uuid> {
        let mut descendents = self.get_children(entity);
        let mut stack_rel: Vec<Uuid> = entity.children_id.clone();

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
        descendents.iter().map(|e| e.get_uuid()).collect()
    }

    /// Gets an entity's descendents as a reference.
    /// # Arguement
    /// - `entity`: A borrow of an entity
    /// # Returns
    /// A collection of entities.
    pub fn get_descendents(&self, entity: &Entity) -> Vec<Ref<Entity>> {
        self.get_descendents_id(entity)
            .iter()
            .map(|id| self.entity_map[id].borrow())
            .collect()
    }

    /// Gets an entity's descendents as an mutable reference.
    /// # Arguements
    /// - `entity`: A borrow of an entity
    /// # Returns
    /// A collection of entities as mutable references.
    pub fn get_descendents_mut(&self, entity: &Entity) -> Vec<RefMut<Entity>> {
        self.get_descendents_id(entity)
            .iter()
            .map(|id| self.entity_map[id].borrow_mut())
            .collect()
    }
}
