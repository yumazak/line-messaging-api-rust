#[derive(Serialize, Deserialize)]
pub enum LineSourceType {
    User { user_id: String },
    Group{ group_id: String , user_id: String},
    Room { room_id: String, user_id: String },
}

#[derive(Serialize, Deserialize)]
pub struct LineSource {
    pub kind: LineSourceType,
}

impl LineSource {
    pub fn new (kind: LineSourceType) -> LineSource {
        LineSource { kind }
    }
}