use std::sync::Arc;

use aion_ecs::prelude::Query;
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError};
use tokio::runtime::Runtime;

use crate::prelude::PipelineResource;

pub trait GetPipelineResources {
    fn get_pipeline_resources(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &PipelineResource>, ProgramRegistryResolveEitherError>;
}

impl GetPipelineResources for Arc<ProgramRegistry> {
    fn get_pipeline_resources(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &PipelineResource>, ProgramRegistryResolveEitherError>
    {
        self.resolve_simple_either::<Query<&PipelineResource>>(runtime)
    }
}