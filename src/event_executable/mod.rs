use std::{collections::{HashMap, HashSet}, sync::Arc};

use aion_ecs::prelude::{Query, World};
use aion_event::prelude::{Event, EventBuffer, EventHistory, EventSystem};
use aion_processor::prelude::SystemId;
use aion_program::prelude::{AccessBuilder, ProgramRegistry, Shared, Unique};
use hecs::Entity;

use crate::{event_executable::event_reactor::EventReactor, prelude::{ExecutableEvent, ExecutablePipeline, PipelineId, PipelineResource, PipelineResources}};

pub mod executable_pipeline;
pub mod pipeline_id;

pub mod executable_event;
pub mod pipeline_resources;
pub mod pipeline_resource;
pub mod get_pipeline_resources;
pub mod event_reactor;

pub struct EventExecutable;

impl EventSystem for EventExecutable {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        _current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let mut next_executables = HashMap::new();
        let mut exhausted_executable_pipelines = HashSet::new();
        {
            let executable_pipelines = program_registry.resolve::<Query<(Entity, &mut ExecutablePipeline, Option<&PipelineId>)>>(None, vec![]);
            if let Ok(Ok(executable_pipelines)) = executable_pipelines {
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

        // for exhausted_pipeline 
        // world.remove

        let mut pipeline_event_map = HashMap::new();
        {
            let executable_events = program_registry.resolve::<Query<&ExecutableEvent>>(None, vec![]);
            if let Ok(Ok(executable_events)) = executable_events {
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


        let mut event_reactors: HashMap<Event, HashSet<SystemId>> = HashMap::new();
        {
            for program_id in program_registry.program_ids() {
                let program_access_builder = AccessBuilder {
                    program_id: Some(program_id.clone()),
                    ..Default::default()
                };
                                
                let world = program_registry.resolve::<Shared<World>>(None, vec![program_access_builder]);
                if let Ok(Ok(world)) = world {
                    let prepared_event_reactors = world.prepare_query::<(Entity, &EventReactor)>();
                    if let Some(prepared_event_reactors) = prepared_event_reactors {
                        for (entity, event_reactor) in prepared_event_reactors.query(&world).iter() {
                            for event in event_reactor.events() {
                                event_reactors.entry(event.clone()).or_default().insert((program_id.clone(), entity));
                            }
                        }
                    }
                }
            }
        }

        {
            for system_ids in event_reactors.values() {
                for (program_id, system_entity) in system_ids {
                    let program_access_builder = AccessBuilder {
                        program_id: Some(program_id.clone()),
                        ..Default::default()
                    };

                    let insert_default = {
                        let world = program_registry.resolve::<Shared<World>>(None, vec![program_access_builder]);
                        if let Ok(Ok(world)) = world {
                            if world.has::<PipelineResources>(*system_entity).is_some_and(|has| has) {
                                let prepared_system_pipeline_resources = world.prepare_get_unique::<PipelineResources>(*system_entity);
                                if let Some(prepared_system_pipeline_resources) = prepared_system_pipeline_resources {
                                    let mut system_pipeline_resources = prepared_system_pipeline_resources.get(&world);
                                    system_pipeline_resources.clear();
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
                        
                        let world = program_registry.resolve::<Unique<World>>(None, vec![program_access_builder]);
                        if let Ok(Ok(mut world)) = world {
                            let _ = world.insert(*system_entity, (PipelineResources::default(),));
                        }
                    }                                
                }
            }
        }

        {
            let pipeline_resources = program_registry.resolve::<Query<&PipelineResource>>(None, vec![]);
            if let Ok(Ok(pipeline_resources)) = pipeline_resources {
                for pipeline_resource in pipeline_resources.query().iter() {
                    if let Some(event) = pipeline_event_map.get(&Some(pipeline_resource.pipeline_id().clone())) {
                        if let Some(systems) = event_reactors.get(event) {
                            for (program_id, system_entity) in systems {
                                let program_access_builder = AccessBuilder {
                                    program_id: Some(program_id.clone()),
                                    ..Default::default()
                                };
                                
                                let world = program_registry.resolve::<Shared<World>>(None, vec![program_access_builder]);
                                if let Ok(Ok(world)) = world {
                                    let prepared_system_pipeline_resources = world.prepare_get_unique::<PipelineResources>(*system_entity);
                                    if let Some(prepared_system_pipeline_resources) = prepared_system_pipeline_resources {
                                        let mut system_pipeline_resources = prepared_system_pipeline_resources.get(&world);
                                        system_pipeline_resources.insert(*pipeline_resource.entity());
                                    }
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
