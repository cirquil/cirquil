use uuid::Uuid;

pub fn make_uuid() -> Uuid {
    Uuid::now_v7()
}
