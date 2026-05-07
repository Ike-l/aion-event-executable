use std::{any::TypeId, sync::Arc};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::{ProgramRegistry, UserId, UserPassword};

use crate::prelude::{get_executable_event_registry, get_mut_executable_pipeline_buffer};

pub mod executable_pipeline_buffer;
pub mod executable_event_registry;

#[cfg(feature = "pipeline-resources")]
pub mod resource_registry;
#[cfg(feature = "pipeline-resources")]
pub mod executable_system_registry;
#[cfg(feature = "pipeline-resources")]
pub mod executable;

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
                    Some(next_executables.into_iter().filter_map(|(id, executable_reference)| {
                        executable_event_registry.as_ref().get(&executable_reference).cloned().and_then(|event| Some((id, event)))
                    }).collect::<Vec<_>>())
                } else {
                    None
                }
            },
            _ => None
        };

        if let Some(new_events) = new_events {
            for (id, new_event) in new_events {
                #[cfg(feature = "pipeline-resources")]
                {
                    use crate::prelude::{get_resource_registry, get_executable_system_registry};
                    use aion_program::prelude::AccessBuilder;
                    use aion_event_processor::prelude::get_system_metadata;

                    match get_resource_registry(program_registry) {
                        Ok(Ok(Ok(resource_registry))) => {
                            let source = resource_registry.as_ref().get(&id);
                            if let Some(Some(source)) = source {

                                match get_executable_system_registry(program_registry) {
                                    Ok(Ok(Ok(executable_system_registry))) => {
                                        if let Some(system_metadata_resource_ids) = executable_system_registry.as_ref().get(&new_event) {
                                            for system_metadata_resource_id in system_metadata_resource_ids {
                                                match get_system_metadata(program_registry, system_metadata_resource_id) {
                                                    Ok(Ok(Ok(mut system_metadata))) => {
                                                        system_metadata.as_mut().insert_access_builder(
                                                            AccessBuilder {
                                                                user_details: Some(EXECUTABLE_USER_DETAILS),
                                                                resource_id: Some(source.clone()),
                                                                ..Default::default()
                                                            }
                                                        )
                                                    },
                                                    _ => ()
                                                }
                                            }
                                        }
                                    },
                                    _ => ()
                                }
                            }
                        },
                        _ => ()
                    }
                }

                event_buffer.insert(new_event);
            }
        }

        event_buffer
    }
}
