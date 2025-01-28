use uuid::Uuid;

pub fn gen_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}
