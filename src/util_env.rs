
use std::env;

pub fn is_env_on(var: &str) -> bool {
    match env::var(var) {
        Err(_) => false,
        Ok(v) => (v == "TRUE" || v == "true" || v =="YES" || v == "yes" || v == "1"),
    }
}
