use uuid::Uuid;

pub trait StringUUIDSupport {
    /// 生成一个32位的小写UUID(V4版本)
    fn gen_id() -> String;
}

impl StringUUIDSupport for String {
    fn gen_id() -> String {
        Uuid::new_v4().to_string().replace("-", "")
    }
}
