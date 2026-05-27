use std::sync::Arc;

use aion_ecs::prelude::Query;
use aion_program::prelude::{AccessBuilder, ProgramRegistry, ProgramRegistryResolveEitherError};
use hecs::Entity;
use tokio::runtime::Runtime;

use crate::prelude::EventReactor;

pub trait GetEventReactors {
    fn get_event_reactors(
        &self,
        runtime: Option<&Runtime>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Query<'_, (Entity, &EventReactor)>, ProgramRegistryResolveEitherError>;
}

impl GetEventReactors for Arc<ProgramRegistry> {
    fn get_event_reactors(
        &self,
        runtime: Option<&Runtime>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Query<'_, (Entity, &EventReactor)>, ProgramRegistryResolveEitherError>
    {
        self.resolve_either::<Query<(Entity, &EventReactor)>>(runtime, access_builders)
    }
}