use std::{collections::HashMap, sync::Arc};

use aion_event::prelude::Event;
use aion_program::prelude::{Shared, AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

pub type ExecutableSystemRegistry = HashMap<Event, Vec<ResourceId>>;

pub const EXECUTABLE_SYSTEM_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventExecutable ExecutableSystemRegistry");

pub const EXECUTABLE_SYSTEM_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(EXECUTABLE_SYSTEM_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_executable_system_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>,
) -> Result<Result<Result<Shared<'a, ExecutableSystemRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    let mut access_builder = EXECUTABLE_SYSTEM_REGISTRY_ACCESS_BUILDER.clone();
    access_builder.program_id = program_id.clone();

    program_registry.resolve_with_insert::<Shared<ExecutableSystemRegistry>>(
        vec![EXECUTABLE_SYSTEM_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ExecutableSystemRegistry::default()))), 
            resource_id: Some(EXECUTABLE_SYSTEM_REGISTRY_RESOURCE_ID), 
            program_id,
            ..Default::default()
        }
    // is only ever None if resource_id or resource is None
    ).unwrap()
}