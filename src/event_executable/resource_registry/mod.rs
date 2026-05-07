use std::{collections::HashMap, sync::Arc};

use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

use crate::prelude::PipelineId;

pub type ResourceRegistry = HashMap<PipelineId, Option<ResourceId>>;

pub const RESOURCE_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventExecutable ResourceRegistry");

pub const RESOURCE_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(RESOURCE_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_resource_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Shared<'a, ResourceRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Shared<ResourceRegistry>>(
        vec![RESOURCE_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ResourceRegistry::default()))), 
            resource_id: Some(RESOURCE_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id is None
    ).unwrap()
}