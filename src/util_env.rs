use std::env;

pub fn is_env_on(var: &str) -> bool {
    env_var(var).map(|val| is_on(&val)).unwrap_or(false)
}

pub fn is_env_off(var: &str) -> bool {
    env_var(var).map(|val| is_off(&val)).unwrap_or(false)
}

pub fn is_on(val: &str) -> bool {
    let lower_val = val.to_lowercase();
    ["true", "yes", "1"].iter().any(|x| *x == lower_val)
}

pub fn is_off(val: &str) -> bool {
    let lower_val = val.to_lowercase();
    ["false", "no", "0"].iter().any(|x| *x == lower_val)
}

pub fn env_var(var: &str) -> Option<String> {
    let var_from_env = env::var(var).ok();
    if var_from_env.is_some() {
        return var_from_env;
    }
    let var_content = crate::util_file::read_file_content(&format!("~/.config/envs/{}", var));
    if let Ok(var_content) = var_content {
        return Some(var_content.trim().to_string());
    }
    None
}
