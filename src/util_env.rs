use std::env;

pub fn is_env_on(var: &str) -> bool {
    env::var(var).ok().map(|val| is_on(&val)).unwrap_or(false)
}

pub fn is_env_off(var: &str) -> bool {
    env::var(var).ok().map(|val| is_off(&val)).unwrap_or(false)
}

pub fn is_on(val: &str) -> bool {
    let lower_val = val.to_lowercase();
    ["true", "yes", "1"].iter().any(|x| *x == lower_val)
}

pub fn is_off(val: &str) -> bool {
    let lower_val = val.to_lowercase();
    ["false", "no", "0"].iter().any(|x| *x == lower_val)
}
