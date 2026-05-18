use std::sync::Arc;

use aion_ecs::prelude::World;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError, Shared};
use hecs::Entity;

use crate::prelude::PipelineResources;

#[derive(small_derive_deref::Deref, small_derive_deref::DerefMut)]
pub struct GetPipelineResources {
    pipeline_resources: PipelineResources
}


impl Injection for GetPipelineResources {
    type Item<'new> = GetPipelineResources;

    fn claim_manual_access_builders(_accesses: Vec<&AccessBuilder>) -> Vec<usize> { vec![] }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        Shared::<World>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let world = Shared::<World>::resolve_access(entity, program_registry, derived_results)?;
        
        let prepared_pipeline_resources = world
            .prepare_get_shared::<PipelineResources>(
                entity
                    .ok_or(ResolveResourceError::Resolving)?
            ).ok_or(ResolveResourceError::Resolving)?;

        let pipeline_resources = prepared_pipeline_resources.get(&world);
        Ok(GetPipelineResources {
            pipeline_resources: (*pipeline_resources).clone()
        })
    }
}