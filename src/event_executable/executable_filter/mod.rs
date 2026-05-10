use aion_processor::prelude::UserDetails;
use aion_program::prelude::{UserId, UserPassword};

use crate::prelude::EXECUTABLE_USER_DETAILS;

pub struct ExecutableFilter;

impl UserDetails for ExecutableFilter {
    fn get_user_details() -> Option<(UserId, UserPassword)> {
        Some(EXECUTABLE_USER_DETAILS)
    }
}
