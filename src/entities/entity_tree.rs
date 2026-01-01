use crate::entities::entity::Entity;
use std::rc::Rc;

pub struct EntityTree<'a> {
    pub head: Rc<Entity<'a>>,
}
