use std::{collections::HashMap, sync::Arc};

use aion_processor::prelude::Unique;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

use crate::prelude::PipelineId;

pub type SystemPipelineRegistry = HashMap<ResourceId, HashMap<PipelineId, Option<usize>>>;

pub const SYSTEM_PIPELINE_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventExecutable SystemPipelineRegistry");

pub const SYSTEM_PIPELINE_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(SYSTEM_PIPELINE_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_system_pipeline_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, SystemPipelineRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<SystemPipelineRegistry>>(
        vec![SYSTEM_PIPELINE_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(SystemPipelineRegistry::default()))), 
            resource_id: Some(SYSTEM_PIPELINE_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}