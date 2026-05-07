use std::{collections::HashMap, sync::Arc};

use aion_event::prelude::Event;
use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

pub type ExecutableEventRegistry = HashMap<String, Event>;

pub const EXECUTABLE_EVENT_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventExecutable ExecutableEventRegistry");

pub const EXECUTABLE_EVENT_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(EXECUTABLE_EVENT_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_executable_event_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Shared<'a, ExecutableEventRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Shared<ExecutableEventRegistry>>(
        vec![EXECUTABLE_EVENT_REGISTRY_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ExecutableEventRegistry::default()))), 
            resource_id: Some(EXECUTABLE_EVENT_REGISTRY_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id or resource is None
    ).unwrap()
}