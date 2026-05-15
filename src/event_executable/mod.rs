use std::{any::TypeId, collections::{HashMap, HashSet}, sync::Arc};

use aion_ecs::prelude::Query;
use aion_event::prelude::{Event, EventBuffer, EventHistory, EventSystem};
use aion_program::prelude::{ProgramRegistry, UserId, UserPassword};
use hecs::Entity;

use crate::prelude::{ExecutablePipeline, ExecutablePipelineBuffer, PipelineId, get_executable_event_registry, get_mut_executable_pipeline_buffer};

pub mod executable_pipeline_buffer;
pub mod executable_event_registry;


#[cfg(feature = "load-access-builders")]
pub mod executable_filter;

#[cfg(feature = "pipeline-resources")]
pub mod executable_system_registry;
#[cfg(feature = "pipeline-resources")]
pub mod system_pipeline_registry;
#[cfg(feature = "pipeline-resources")]
pub mod resource_registry;

pub struct EventExecutable;

pub const EXECUTABLE_USER_DETAILS: (UserId, UserPassword) = (UserId::TypeId(TypeId::of::<EventExecutable>()), UserPassword::TypeId(TypeId::of::<EventExecutable>()));

impl EventSystem for EventExecutable {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        _current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let mut next_executables = Vec::new();
        let mut exhausted_executable_pipelines = HashSet::new();
        {
            let executable_pipelines = program_registry.resolve::<Query<(Entity, &mut ExecutablePipeline)>>(vec![]);
            if let Ok(Ok(mut executable_pipelines)) = executable_pipelines {
                for (entity, executable_pipeline) in executable_pipelines.borrow().iter() {
                    if let Some(next_executable) = executable_pipeline.pop_front() {
                        if let Some(next_executable) = next_executable {
                            next_executables.push(next_executable);
                        }
                    } else {
                        exhausted_executable_pipelines.insert(entity);
                    }
                }
            }
        }

        struct ExecutableEvent {
            id: String,
            event: Event
        }

        {
            let executable_events = program_registry.resolve::<Query<&ExecutableEvent>>(vec![]);
            if let Ok(Ok(executable_events)) = executable_events {
                for executable_event in executable_events.borrow().iter() {
                    if next_executables.contains(&&executable_event.id) {
                        event_buffer.insert(executable_event.event.clone());
                    }
                }
            }
        }

        {
            // for each program
            // find pipeline_id -> resource_id/entity

            // find event -> system_resource_id/entity
            

            // put resource_id/entity into the system's entity
            // as many PipelineId -> Entity

            // then maybe get the system's entity in Injection
            // can then fetch the resource_id for it

        }

        #[cfg(feature = "pipeline-resources")]
        {
            use aion_program::prelude::{ResourceId, AccessBuilder};
            use crate::prelude::{get_resource_registry, get_executable_system_registry, get_system_pipeline_registry, SystemPipelineRegistry};

            for program_id in program_registry.program_ids() {
                let resolved_pipelines: Option<Vec<(&Event, &PipelineId, ResourceId)>> = if let Some(new_events) = new_events.as_ref() {
                    match get_resource_registry(program_registry, Some(program_id.clone())) {
                        Ok(Ok(Ok(resource_registry))) => {
                            let mut resolved = Vec::new();
        
                            for (event, pipelines) in new_events {
                                for pipeline in pipelines {
                                    if let Some(Some(resource)) = resource_registry.as_ref().get(&pipeline) {
                                        resolved.push((event, pipeline, resource.clone()));
                                    }
                                }
                            }
        
                            Some(resolved)
                        }
                        _ => None,
                    }
                } else { None };
        
                let resolved_systems: Option<Vec<(ResourceId, &PipelineId, ResourceId)>> = if let Some(resolved_pipelines) = resolved_pipelines {
                    match get_executable_system_registry(program_registry, Some(program_id.clone())) {
                        Ok(Ok(Ok(executable_system_registry))) => {
                            let mut resolved = Vec::new();
            
                            for (event, pipeline, resource) in resolved_pipelines {
                                if let Some(systems) = executable_system_registry.as_ref().get(&event) {
                                    for system in systems {
                                        resolved.push((
                                            system.clone(),
                                            pipeline,
                                            resource.clone(),
                                        ));
                                    }
                                }
                            }
            
                            Some(resolved)
                        }
                        _ => None,
                    }
                } else { None };
        
                let mut new_system_pipeline_registry: HashMap<ResourceId, HashMap<PipelineId, Option<usize>>> = SystemPipelineRegistry::new();
                if let Some(resolved_systems) = resolved_systems {
                    for (system, pipeline, resource) in resolved_systems {
                        // feature flag here
                        #[cfg(feature = "load-access-builders")]
                        let index = {
                            use aion_event_processor::prelude::get_mut_system_metadata;
                            match get_mut_system_metadata(program_registry, Some(program_id.clone()), system.clone()) {
                                Ok(Ok(mut system_metadata)) => {
                                    let index = system_metadata.as_ref().stored_access_builders().len();
                
                                    system_metadata.as_mut().insert_access_builder(AccessBuilder {
                                        resource_id: Some(resource.clone()),
                                        program_id: Some(program_id.clone()),
                                        ..Default::default()
                                    });
                                    
                                    Some(index)
                                }
                                _ => None
                            }
                        };
        
                        #[cfg(not(feature = "load-access-builders"))]
                        let index = None;
        
                        assert!(
                            new_system_pipeline_registry
                                .entry(system)
                                .or_default()
                                .insert((*pipeline).clone(), index)
                                .is_none()
                        );
                    }
                }
        
                if let Ok(Ok(Ok(mut system_pipeline_registry))) = get_system_pipeline_registry(program_registry, Some(program_id.clone())) {
                    *system_pipeline_registry.as_mut() = new_system_pipeline_registry;
                }
            }
        }


        event_buffer
    }
}
