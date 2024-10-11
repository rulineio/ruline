use rand::{distributions::Alphanumeric, Rng};
use tracing::error;

pub fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub trait ResultExt<T> {
    fn log_error(self, msg: &'static str) -> Self;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: std::fmt::Display,
{
    fn log_error(self, msg: &'static str) -> Self {
        if let Err(e) = &self {
            error!({ error = %e }, "{}", msg);
        }

        self
    }
}
