use std::sync::Arc;

use aion_ecs::prelude::Query;
use aion_program::prelude::{ProgramRegistry, ProgramRegistryResolveEitherError};
use tokio::runtime::Runtime;

use crate::prelude::ExecutableEvent;

pub trait GetExecutableEvents {
    fn get_executable_events(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &ExecutableEvent>, ProgramRegistryResolveEitherError>;
}

impl GetExecutableEvents for Arc<ProgramRegistry> {
    fn get_executable_events(
        &self,
        runtime: Option<&Runtime>
    ) -> Result<Query<'_, &ExecutableEvent>, ProgramRegistryResolveEitherError>
    {
        self.resolve_simple_either::<Query<&ExecutableEvent>>(runtime)
    }
}