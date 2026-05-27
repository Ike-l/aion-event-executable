use std::{collections::{HashMap, HashSet}, sync::Arc};

use aion_event::prelude::{Event, EventBuffer, EventHistory, EventSystem};
use aion_processor::prelude::SystemId;
use aion_program::prelude::ProgramRegistry;

use crate::prelude::PipelineId;

#[cfg(feature = "any")]
use tokio::runtime::Runtime;

#[cfg(feature = "any")]
use aion_program::prelude::Shared;

#[cfg(any(feature = "processing", feature = "load-pipeline-resources"))]
use aion_ecs::prelude::World;
#[cfg(any(feature = "processing", feature = "load-pipeline-resources"))]
use aion_program::prelude::Unique;

#[cfg(feature = "processing")]
pub mod executable_pipeline;
#[cfg(feature = "processing")]
pub mod get_executable_pipelines;
#[cfg(feature = "processing")]
use crate::prelude::{GetExecutablePipelines, ExecutablePipeline};

pub mod pipeline_id;

#[cfg(feature = "pipeline-events")]
pub mod executable_event;
#[cfg(feature = "pipeline-events")]
pub mod get_executable_events;

#[cfg(any(feature = "load-pipeline-resources", feature = "event-reactors"))]
use aion_program::prelude::{AccessBuilder};

#[cfg(feature = "event-reactors")]
pub mod event_reactor;
#[cfg(feature = "event-reactors")]
pub mod get_event_reactors;

#[cfg(feature = "load-pipeline-resources")]
use crate::prelude::{PipelineResources, GetUniquePipelineResources};
#[cfg(feature = "load-pipeline-resources")]
pub mod pipeline_resources;
#[cfg(feature = "load-pipeline-resources")]
pub mod pipeline_resource;
#[cfg(feature = "load-pipeline-resources")]
pub mod inject_pipeline_resources;
#[cfg(feature = "load-pipeline-resources")]
pub mod get_unique_pipeline_resources;
#[cfg(feature = "load-pipeline-resources")]
pub mod get_pipeline_resources;

pub struct EventExecutable;

impl EventSystem for EventExecutable {
    fn execute(
        &self,
        #[allow(unused_variables)]
        program_registry: &Arc<ProgramRegistry>, 
        _current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        #[allow(unused)]
        let mut event_buffer = EventBuffer::default();

        #[cfg(feature = "any")]
        let runtime = program_registry.resolve::<Shared<Runtime>>(None, vec![]);
        #[cfg(feature = "any")]
        let runtime = match runtime {
            Ok(runtime) => Some(runtime),
            _ => None
        };

        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut next_executables: HashMap<Option<PipelineId>, String> = HashMap::new();
        #[allow(unused_mut)]
        #[allow(unused_variables)]
        let mut exhausted_executable_pipelines: HashSet<hecs::Entity> = HashSet::new();
        #[cfg(feature = "processing")]
        {            
            let executable_pipelines = program_registry.get_executable_pipelines(runtime.as_deref());
            if let Ok(executable_pipelines) = executable_pipelines {
                for (entity, executable_pipeline, pipeline_id) in executable_pipelines.query().iter() {
                    if let Some(next_executable) = executable_pipeline.pop_front().cloned() {
                        if let Some(next_executable) = next_executable {
                            next_executables.insert(pipeline_id.cloned(), next_executable);
                        }
                    } else {
                        exhausted_executable_pipelines.insert(entity);
                    }
                }
            }
        }

        #[cfg(feature = "processing")]
        {   
            let world = program_registry.resolve_simple_either::<Unique<World>>(runtime.as_deref());
            if let Ok(mut world) = world {
                for exhausted_pipeline in exhausted_executable_pipelines {
                    // separate bc if it dont have PipelineId it wouldnt remove ExecutablePipeline
                    let _ = world.remove::<(ExecutablePipeline,)>(exhausted_pipeline);
                    let _ = world.remove::<(PipelineId,)>(exhausted_pipeline);
                }
            }
        }

        #[allow(unused)]
        let mut pipeline_event_map: HashMap<&Option<PipelineId>, Event> = HashMap::new();
        #[cfg(feature = "pipeline-events")]
        {
            use crate::prelude::GetExecutableEvents;

            let executable_events = program_registry.get_executable_events(runtime.as_deref());
            if let Ok(executable_events) = executable_events {
                for executable_event in executable_events.query().iter() {
                    for (pipeline_id, executable) in next_executables.iter() {
                        if executable == executable_event.id() {
                            event_buffer.insert(executable_event.event().clone());
                            pipeline_event_map.insert(pipeline_id, executable_event.event().clone());
                        }
                    }
                }
            }
        }


        #[allow(unused)]
        let mut current_event_reactors: HashMap<Event, HashSet<SystemId>> = HashMap::new();
        #[cfg(feature = "event-reactors")]
        {
            for program_id in program_registry.program_ids() {
                use crate::prelude::GetEventReactors;

                let program_access_builder = AccessBuilder {
                    program_id: Some(program_id.clone()),
                    ..Default::default()
                };
                                
                let event_reactors = program_registry.get_event_reactors(runtime.as_deref(), vec![program_access_builder]);
                if let Ok(event_reactors) = event_reactors {
                    for (entity, event_reactor) in event_reactors.query().iter() {
                        for event in event_reactor.events() {
                            current_event_reactors.entry(event.clone()).or_default().insert((program_id.clone(), entity));
                        }
                    }
                }
            }
        }

        #[cfg(feature = "load-pipeline-resources")]
        {
            for system_ids in current_event_reactors.values() {
                for (program_id, system_entity) in system_ids {
                    let program_access_builder = AccessBuilder {
                        program_id: Some(program_id.clone()),
                        ..Default::default()
                    };

                    let insert_default = {
                        let world = program_registry.resolve_either::<Shared<World>>(runtime.as_deref(), None, vec![program_access_builder.clone()]);
                        if let Ok(world) = world {
                            if world.has::<PipelineResources>(*system_entity).is_some_and(|has| has) {
                                drop(world);
                                
                                let pipeline_resource = program_registry.get_unique_pipeline_resources(runtime.as_deref(), *system_entity, vec![program_access_builder]);
                                if let Ok(pipeline_resource) = pipeline_resource {
                                    pipeline_resource.get_unique().clear();
                                }

                                false
                            } else { true }
                        } else { true }
                    };
                    
                    if insert_default {
                        let program_access_builder = AccessBuilder {
                            program_id: Some(program_id.clone()),
                            ..Default::default()
                        };
                        
                        let world = program_registry.resolve_either::<Unique<World>>(runtime.as_deref(), None, vec![program_access_builder]);
                        if let Ok(mut world) = world {
                            let _ = world.insert(*system_entity, (PipelineResources::default(),));
                        }
                    }                                
                }
            }
        }

        #[cfg(feature = "load-pipeline-resources")]
        {
            use crate::prelude::GetPipelineResources;

            let pipeline_resources = program_registry.get_pipeline_resources(runtime.as_deref());
            if let Ok(pipeline_resources) = pipeline_resources {
                for pipeline_resource in pipeline_resources.query().iter() {
                    if let Some(event) = pipeline_event_map.get(&Some(pipeline_resource.pipeline_id().clone())) {
                        if let Some(systems) = current_event_reactors.get(event) {
                            for (program_id, system_entity) in systems {
                                let program_access_builder = AccessBuilder {
                                    program_id: Some(program_id.clone()),
                                    ..Default::default()
                                };
                                
                                let pipeline_resources = program_registry.get_unique_pipeline_resources(runtime.as_deref(), *system_entity, vec![program_access_builder]);
                                if let Ok(pipeline_resources) = pipeline_resources {
                                    pipeline_resources.get_unique().insert(*pipeline_resource.entity());
                                }
                            }
                        }
                    }
                }
            }
        }

        event_buffer
    }
}
