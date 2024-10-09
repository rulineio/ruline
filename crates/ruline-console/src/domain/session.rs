use super::{member::Member, organization::Organization, user::User};

#[derive(Clone)]
pub enum Session {
    Oauth {
        state: String,
    },
    Unauthenticated {
        state: String,
        user: User,
    },
    User {
        user: User,
    },
    Member {
        user: User,
        organization: Organization,
        member: Member,
    },
}

mod builder {
    use super::*;

    #[derive(Default)]
    pub struct Builder {
        state: Option<String>,
        user: Option<User>,
        organization: Option<Organization>,
        member: Option<Member>,
    }

    impl Builder {
        #[must_use]
        pub fn state(mut self, state: String) -> Self {
            self.state = Some(state);
            self
        }

        #[must_use]
        pub fn user(mut self, user: User) -> Self {
            self.user = Some(user);
            self
        }

        #[must_use]
        pub fn organization(mut self, organization: Organization) -> Self {
            self.organization = Some(organization);
            self
        }

        #[must_use]
        pub fn member(mut self, member: Member) -> Self {
            self.member = Some(member);
            self
        }

        pub fn build(self) -> Session {
            match (self.state, self.user, self.organization, self.member) {
                (Some(state), None, None, None) => Session::Oauth { state },
                (Some(state), Some(user), None, None) => Session::Unauthenticated { state, user },
                (None, Some(user), None, None) => Session::User { user },
                (None, Some(user), Some(organization), Some(member)) => Session::Member {
                    user,
                    organization,
                    member,
                },
                _ => panic!("invalid session"),
            }
        }
    }
}

impl Session {
    pub fn builder() -> builder::Builder {
        builder::Builder::default()
    }
}
