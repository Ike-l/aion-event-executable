use std::{any::TypeId, collections::HashMap, sync::Arc};

use aion_event::prelude::{Event, EventBuffer, EventHistory, EventSystem};
use aion_event_processor::prelude::get_system_metadata;
use aion_program::prelude::{AccessBuilder, ProgramRegistry, UserId, UserPassword};

use crate::prelude::{PipelineId, SystemPipelineRegistry, get_executable_event_registry, get_executable_system_registry, get_mut_executable_pipeline_buffer, get_resource_registry, get_system_pipeline_registry};

pub mod executable_pipeline_buffer;
pub mod executable_event_registry;


// #[cfg(feature = "pipeline-resources")]
pub mod executable_system_registry;
// #[cfg(feature = "pipeline-resources")]
pub mod executable;

// actually have a different feature flag
// #[cfg(feature = "pipeline-resources")]
pub mod system_pipeline_registry;
// #[cfg(feature = "pipeline-resources")]
pub mod resource_registry;

pub struct EventExecutable;

pub const EXECUTABLE_USER_DETAILS: (UserId, UserPassword) = (UserId::TypeId(TypeId::of::<EventExecutable>()), UserPassword::TypeId(TypeId::of::<EventExecutable>()));

impl EventSystem for EventExecutable {
    fn execute(
        program_registry: &Arc<ProgramRegistry>, 
        _current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let next_executables = match get_mut_executable_pipeline_buffer(program_registry) {
            Ok(Ok(Ok(mut executable_pipeline_buffer))) => {
                Some(executable_pipeline_buffer.as_mut().next())
            },
            _ => None,
        };

        let new_events = match get_executable_event_registry(program_registry) {
            Ok(Ok(Ok(executable_event_registry))) => {
                if let Some(next_executables) = next_executables {
                    let mut events: HashMap<Event, Vec<PipelineId>> = HashMap::new();
                    for (id, executable_reference) in next_executables {
                        let event = executable_event_registry.as_ref().get(&executable_reference);
                        if let Some(event) = event {
                            event_buffer.insert(event.clone());
                            events.entry(event.clone()).or_default().push(id);
                        }
                    }

                    Some(events)
                } else {
                    None
                }
            },
            _ => None
        };

        // TODO separate this into 4 sections
        // So that each "get" can fail and the other systems will still behave
        // TODO
        // feature flags for each section
        // work out which feature flag depends on which other

        
        let mut new_system_pipeline_registry = SystemPipelineRegistry::new();
        if let Some(new_events) = new_events {
            for (event, pipelines) in new_events {
                if let Ok(Ok(Ok(resource_registry))) = get_resource_registry(program_registry) {
                    for pipeline in pipelines {
                        if let Ok(Ok(Ok(executable_system_registry))) = get_executable_system_registry(program_registry) {
                            if let Some(systems) = executable_system_registry.as_ref().get(&event) {
                                for system in systems {
                                    if let Ok(Ok(Ok(mut system_metadata))) = get_system_metadata(program_registry, system) {
                                        if let Some(Some(source)) = resource_registry.as_ref().get(&pipeline) {
                                            let index = system_metadata.as_ref().stored_access_builders().len();
                                            system_metadata.as_mut().insert_access_builder(AccessBuilder {
                                                resource_id: Some(source.clone()),
                                                ..Default::default()
                                            });

                                            assert!(new_system_pipeline_registry
                                                .entry(system.clone())
                                                .or_default()
                                                .insert(pipeline.clone(), Some(index)).is_none());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Ok(Ok(Ok(mut system_pipeline_registry))) = get_system_pipeline_registry(program_registry) {
            *system_pipeline_registry.as_mut() = new_system_pipeline_registry;
        }

        event_buffer
    }
}
