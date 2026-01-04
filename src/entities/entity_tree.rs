use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use uuid::Uuid;

use crate::entities::entity::{Entity, EntityType, GameGenre, GameType};

#[derive(Debug, Default)]
pub struct EntityTree {
    pub head: Option<Uuid>,
    entity_map: HashMap<Uuid, Rc<RefCell<Entity>>>,
}
impl EntityTree {
    pub fn add_entity(&mut self, name: &str, entity_type: EntityType) -> Rc<RefCell<Entity>> {
        let entity = Rc::new(RefCell::new(Entity::new(name, entity_type)));
        let id = entity.borrow().get_uuid();
        self.entity_map.insert(id, entity.clone());
        entity
    }

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

    pub fn get_entity(&self, id: Uuid) -> Option<Ref<Entity>> {
        let entity_null = self.entity_map.get(&id);
        if let Some(entity) = entity_null {
            return Some(entity.borrow());
        }
        None
    }

    pub fn get_entity_mut(&self, id: Uuid) -> Option<RefMut<Entity>> {
        let entity_null = self.entity_map.get(&id);
        if let Some(entity) = entity_null {
            return Some(entity.borrow_mut());
        }
        None
    }

    // Parent

    pub fn get_parent(&self, entity: &Entity) -> Option<Ref<Entity>> {
        let id = entity.parent_id?;

        let relative_null = self.entity_map.get(&id);

        if let Some(relative) = relative_null {
            return Some(relative.borrow());
        }
        None
    }

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

    pub fn set_parent(
        &mut self,
        mut entity: RefMut<Entity>,
        parent: Option<RefMut<Entity>>,
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

    // Children

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
    // WHY THE FUCK DOES USING THIS * ON A VEC RETURN A FUCKING SLICE
    // New contender on worst programming language?
    // Favourite thing in this project is not coding for days only design because I feel like Sir
    // Francis Drake the way I am circumnavgating the borrow checker
    // Great way to start of the year, Graydon Hoare
    // Fuck it reinventing the Von Newman Archietchure just to avoid this abombination of a
    // language

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

    // Rust has to be the worst programming languages of all time
    // why the fuck does this not work
    // death to borrow checker

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
