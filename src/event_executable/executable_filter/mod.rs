use aion_processor::prelude::UserDetailsGetter;
use aion_program::prelude::{UserId, UserPassword};

use crate::prelude::EXECUTABLE_USER_DETAILS;

pub struct ExecutableFilter;

impl UserDetailsGetter for ExecutableFilter {
    fn get_user_details() -> Option<(UserId, UserPassword)> {
        Some(EXECUTABLE_USER_DETAILS)
    }
}
