
use std::env;

pub fn is_env_on(var: &str) -> bool {
    env::var(var).map(|v| v.to_lowercase()).map(|v| vec!["true", "yes", "1"].iter().any(|x| x == &v)).unwrap_or(false)
}
