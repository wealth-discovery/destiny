pub fn has_github_action_env() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok()
}
