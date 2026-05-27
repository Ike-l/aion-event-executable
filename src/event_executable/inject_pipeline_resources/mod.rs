use std::sync::Arc;

use aion_ecs::prelude::GetShared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, DerivedResult, FinalisedAccess, Injection, ProgramRegistry, ResolveResourceError};
use hecs::Entity;

use crate::prelude::PipelineResources;

#[derive(small_derive_deref::Deref, small_derive_deref::DerefMut)]
pub struct InjectPipelineResources {
    pipeline_resources: PipelineResources
}

impl Injection for InjectPipelineResources {
    type Item<'new> = InjectPipelineResources;

    fn claim_manual_access_builders(accesses: Vec<&AccessBuilder>) -> Vec<usize> { GetShared::<PipelineResources>::claim_manual_access_builders(accesses) }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        GetShared::<PipelineResources>::submit_access(prompted_accesses)
    }

    fn resolve_access<'new>(entity: Option<Entity>, program_registry: Arc<ProgramRegistry>, derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let pipeline_resources = GetShared::<PipelineResources>::resolve_access(entity, program_registry, derived_results)?;

        Ok(InjectPipelineResources {
            pipeline_resources: (*pipeline_resources.get_shared()).clone()
        })
    }
}