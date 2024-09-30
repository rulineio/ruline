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

#[derive(Clone, Serialize, Deserialize)]
pub struct PreSession {
    pub state: String,
    pub user: Option<user::User>,
}

mod pre_builder {
    use super::*;

    #[derive(Default)]
    pub struct Builder {
        state: Option<String>,
        user: Option<user::User>,
    }

    impl Builder {
        #[must_use]
        pub fn state(mut self, state: String) -> Self {
            self.state = Some(state);
            self
        }

        #[must_use]
        pub fn user(mut self, user: user::User) -> Self {
            self.user = Some(user);
            self
        }

        pub fn build(self) -> PreSession {
            PreSession {
                state: self.state.expect("state is required"),
                user: self.user,
            }
        }
    }
}

impl PreSession {
    pub fn builder() -> pre_builder::Builder {
        pre_builder::Builder::default()
    }
}
