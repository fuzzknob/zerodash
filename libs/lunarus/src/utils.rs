use colored::Colorize;

pub fn get_required_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| {
        panic!(
            "{}",
            format!("{} env variable is not set", key)
                .on_bright_red()
        )
    })
}

pub fn get_env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}
