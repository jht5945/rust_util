
use std::env;

pub fn is_env_on(var: &str) -> bool {
    env::var(var).map(|v| v.to_lowercase()).map(|v| (v == "true" || v == "yes" || v == "1")).unwrap_or(false)
}
