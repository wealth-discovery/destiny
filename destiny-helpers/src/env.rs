pub trait BoolEnvSupport {
    /// 当前的环境是否在Github Action中
    fn has_github_action() -> bool;
}

impl BoolEnvSupport for bool {
    fn has_github_action() -> bool {
        std::env::var("GITHUB_ACTIONS").is_ok()
    }
}
