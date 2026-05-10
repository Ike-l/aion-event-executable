pub mod event_executable;

pub mod prelude {
    pub use super::{
        event_executable::{
            EXECUTABLE_USER_DETAILS,
            executable_pipeline_buffer::{
                ExecutablePipelineBuffer,
                EXECUTABLE_PIPELINE_BUFFER_ACCESS_BUILDER,
                EXECUTABLE_PIPELINE_BUFFER_RESOURCE_ID,
                get_mut_executable_pipeline_buffer,
                executable_pipeline::{
                    ExecutablePipeline,
                },
                pipeline_id::{
                    PipelineId
                },
            },
            executable_event_registry::{
                EXECUTABLE_EVENT_REGISTRY_ACCESS_BUILDER,
                EXECUTABLE_EVENT_REGISTRY_RESOURCE_ID,
                ExecutableEventRegistry,
                get_executable_event_registry
            },
        },
    };

    #[cfg(feature = "pipeline-resources")]
    pub use super::{
        event_executable::{
            resource_registry::{
                RESOURCE_REGISTRY_ACCESS_BUILDER,
                RESOURCE_REGISTRY_RESOURCE_ID,
                ResourceRegistry,
                get_resource_registry
            },
            executable_system_registry::{
                EXECUTABLE_SYSTEM_REGISTRY_ACCESS_BUILDER,
                EXECUTABLE_SYSTEM_REGISTRY_RESOURCE_ID,
                ExecutableSystemRegistry,
                get_executable_system_registry
            },
            system_pipeline_registry::{
                SYSTEM_PIPELINE_REGISTRY_ACCESS_BUILDER,
                SYSTEM_PIPELINE_REGISTRY_RESOURCE_ID,
                SystemPipelineRegistry,
                get_system_pipeline_registry
            }
        }
    };

    #[cfg(feature = "load-access-builders")]
    pub use super::{
        event_executable::{
            executable_filter::{
                ExecutableFilter
            }
        }
    };
}