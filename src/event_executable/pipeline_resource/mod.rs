use hecs::Entity;

use crate::prelude::PipelineId;

pub struct PipelineResource {
    pipeline_id: PipelineId,
    entity: Entity,
}

impl PipelineResource {
    pub fn pipeline_id(&self) -> &PipelineId {
        &self.pipeline_id
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }
}