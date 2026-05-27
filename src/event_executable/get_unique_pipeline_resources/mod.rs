use std::sync::Arc;

use aion_ecs::prelude::GetUnique;
use aion_program::prelude::{AccessBuilder, ProgramRegistry, ProgramRegistryResolveEitherError};
use hecs::Entity;
use tokio::runtime::Runtime;

use crate::prelude::PipelineResources;

pub trait GetUniquePipelineResources {
    fn get_unique_pipeline_resources(
        self: &Self, 
        runtime: Option<&Runtime>,
        entity: Entity,
        access_builders: Vec<AccessBuilder>
    ) -> Result<GetUnique<'_, PipelineResources>, ProgramRegistryResolveEitherError>;
}

impl GetUniquePipelineResources for Arc<ProgramRegistry> {
    fn get_unique_pipeline_resources(
        self: &Self, 
        runtime: Option<&Runtime>,
        entity: Entity,
        access_builders: Vec<AccessBuilder>
    ) -> Result<GetUnique<'_, PipelineResources>, ProgramRegistryResolveEitherError> {
        self.resolve_either::<GetUnique<PipelineResources>>(runtime, Some(entity), access_builders)
    }
}