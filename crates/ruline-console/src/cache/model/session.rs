use serde::{Deserialize, Serialize};

use crate::db::user;

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub user: user::User,
}

mod builder {
    use super::*;

    #[derive(Default)]
    pub struct Builder {
        user: Option<user::User>,
    }

    impl Builder {
        #[must_use]
        pub fn user(mut self, user: user::User) -> Self {
            self.user = Some(user);
            self
        }

        pub fn build(self) -> Session {
            Session {
                user: self.user.expect("user is required"),
            }
        }
    }
}

impl Session {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
