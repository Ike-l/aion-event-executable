use std::collections::HashSet;

use hecs::Entity;

#[derive(Default, Clone)]
pub struct PipelineResources { 
    entities: HashSet<Entity> 
}

impl PipelineResources {
    pub fn insert(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    pub fn clear(&mut self) {
        self.entities.clear()
    }
}