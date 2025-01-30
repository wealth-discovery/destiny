/// 当前的环境是否在Github Action中
pub fn has_github_action_env() -> bool {
    std::env::var("GITHUB_ACTIONS").is_ok()
}
