// ExecutablePipeline also stores a label?
// And a source/target pair - where the data comes from, and where the system should put the data for the next executable


// also each executable can pass resources to each other,
// they do this by passing ResourceIds 
// how could i extend this to pass EntityIds for an ECS?



// otherwise it is stored in a buffer, the buffer has the Label: <Source: ResourceId (/EntityId)>
// the function can replace the source, or just keep it as is


// doesnt work unless opt-in by providing the resource id for the system metadata
// these pointers are put in the SystemMetadata access builder for the resource
// and provide an InjectedType to then use these ids

// could have a plugin system separate from the event system to do this?


// Could have multiple systems respond to the same event?
// Could have the same system run for different pipelines (So need to ensure each system iterates over each Source corresponding to the event/executable it is reacting to)

// If systems just accept a Executable<Shared<Input>> then it can automagically get the data from the previous executable because, as long as they are all the same type.
// * As long as the systemmetadata resource id pointer is public/registered with this crate
// Executable can iterate over each access builder, select the provided resource id, and pass that to Shared, and then build a Vec of Shared Input


// All a pipeline is:
// Each executable spawns the corresponding event
// Systems can be triggered by these events

// These systems can then fetch the resource ids from the previous executable from the Buffer
// Then they can get the resources

// Or
// These systems can opt into having the ids put automagically into their access builders
// So they can use an Injected parameter to fetch the data directly

// feature flag (extended) requires "Processor Event System" to provide HashSet<SystemMetadata>
// "Processor Event System" has feature flag (blocking) and (non-blocking)

pub mod event_executable;

pub mod prelude {
    pub use super::{
        event_executable::{
            EXECUTABLE_USER_DETAILS,
            EventExecutable,
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
            }
        }
    };
}