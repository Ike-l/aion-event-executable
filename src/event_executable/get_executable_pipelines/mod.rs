use std::sync::Arc;

use aion_ecs::prelude::Query;
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError};
use hecs::Entity;
use tokio::runtime::Runtime;

use crate::prelude::{ExecutablePipeline, PipelineId};

pub trait GetExecutablePipelines {
    fn get_executable_pipelines(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, (Entity, &mut ExecutablePipeline, Option<&PipelineId>)>, ProgramRegistryResolveEitherError>;
}

impl GetExecutablePipelines for Arc<ProgramRegistry> {
    fn get_executable_pipelines(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, (Entity, &mut ExecutablePipeline, Option<&PipelineId>)>, ProgramRegistryResolveEitherError>
    {
        self.resolve_simple_either::<Query<(Entity, &mut ExecutablePipeline, Option<&PipelineId>)>>(runtime)
    }
}