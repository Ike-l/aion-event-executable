use std::collections::HashMap;

use aion_program::prelude::{Injection, AccessBuilder, FinalisedAccess, DerivedResult, ResolveResourceError, AccessSubmissionError};

use crate::prelude::EXECUTABLE_USER_DETAILS;

// needs to be used with Numbered Injection for claim indexes

pub struct Executable<'a, T: Injection> {
    resources: HashMap<usize, T::Item<'a>>
}

impl<'a, T: Injection> Executable<'a, T> {
    pub fn get_resources(&self) -> &HashMap<usize, T::Item<'a>> {
        &self.resources
    }

    pub fn get_mut_resources(&mut self) -> &mut HashMap<usize, T::Item<'a>> {
        &mut self.resources
    }
}

impl<'a, T: Injection> Injection for Executable<'a, T> {
    type Item<'new> = Executable<'new, T>;

    fn claim_indexes(access_builders: Vec<&AccessBuilder>) -> Vec<usize> {
        T::claim_indexes(access_builders)
    }

    fn submit_access(prompted_accesses: Vec<AccessBuilder>) -> Result<Vec<FinalisedAccess>, AccessSubmissionError> {
        let mut finalised_accesses = Vec::new();
        for prompted_access in prompted_accesses {
            if prompted_access.user_details.as_ref().is_some_and(|user_details| *user_details == EXECUTABLE_USER_DETAILS) {
                match T::submit_access(vec![prompted_access]) {
                    Ok(injection_finalised_accesses) => {
                        finalised_accesses.extend(injection_finalised_accesses);
                    },
                    _ => ()
                }
            }
        }

        Ok(finalised_accesses)
    }

    fn resolve_access<'new>(derived_results: Vec<DerivedResult<'new>>) -> Result<Self::Item<'new>, ResolveResourceError> {
        let mut resources = HashMap::new();
        for (i, derived_result) in derived_results.into_iter().enumerate() {
            if let Some(user_details) = derived_result.user_details() {
                if user_details.as_ref().is_some_and(|user_details| *user_details == EXECUTABLE_USER_DETAILS) {
                    let injection_resolved_access = T::resolve_access(vec![derived_result]);
                    if let Ok(resolved_access) = injection_resolved_access {
                        resources.insert(i, resolved_access);
                    }
                }
            }
        }

        Ok(Executable { resources })
    }
}
