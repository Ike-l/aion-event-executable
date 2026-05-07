use std::{collections::HashMap, sync::Arc};

use aion_processor::prelude::Unique;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

use crate::prelude::{ExecutablePipeline, PipelineId};

pub mod executable_pipeline;
pub mod pipeline_id;

#[derive(Default)]
pub struct ExecutablePipelineBuffer {
    buffer: HashMap<PipelineId, ExecutablePipeline>,
    current_id: Option<PipelineId>,
}

impl ExecutablePipelineBuffer {
    pub fn insert(&mut self, pipeline: ExecutablePipeline) {
        let next_id = if let Some(current_id) = &self.current_id {
            match current_id {
                PipelineId::Number(id_number) => PipelineId::Number(id_number + 1),
            }
        } else {
            PipelineId::Number(0)
        };

        self.buffer.insert(next_id, pipeline);
    }

    pub fn next(&mut self) -> Vec<(PipelineId, String)> {
        let mut next_buffer = Vec::new();

        let mut continuing_pipelines = HashMap::new();
        for (id, mut pipeline) in self.buffer.drain() {
            if let Some(executable_reference) = pipeline.pop_front() {
                if let Some(executable_reference) = executable_reference {
                    next_buffer.push((id.clone(), executable_reference.clone()));
                }

                continuing_pipelines.insert(id, pipeline);
            }
        }
        


        next_buffer
    }
}

pub const EXECUTABLE_PIPELINE_BUFFER_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventExecutable ExecutablePipelineBuffer");

pub const EXECUTABLE_PIPELINE_BUFFER_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(EXECUTABLE_PIPELINE_BUFFER_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_executable_pipeline_buffer<'a>(
    program_registry: &'a Arc<ProgramRegistry>
) -> Result<Result<Result<Unique<'a, ExecutablePipelineBuffer>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<ExecutablePipelineBuffer>>(
        vec![EXECUTABLE_PIPELINE_BUFFER_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ExecutablePipelineBuffer::default()))), 
            resource_id: Some(EXECUTABLE_PIPELINE_BUFFER_RESOURCE_ID), 
            ..Default::default()
        }
    // is only ever None if resource_id or resource is None
    ).unwrap()
}