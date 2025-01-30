use uuid::Uuid;

/// 生成一个32位的小写UUID(V4版本)
pub fn gen_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}
