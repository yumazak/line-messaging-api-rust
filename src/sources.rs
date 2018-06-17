#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LineSourceType {
    User  { 
        #[serde(rename = "userId")]
        user_id: String 
    },
    Room  { 
        #[serde(rename = "roomId")]
        room_id: String, 
        #[serde(rename = "userId")]
        user_id: String,
    },
    Group {
        #[serde(rename = "groupId")]
        group_id: String,
        #[serde(rename = "userId", default)]
        user_id:  String
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LineSource {
    #[serde(flatten, rename = "type")]
    pub kind: LineSourceType,
}

impl LineSource {
    pub fn new (kind: LineSourceType) -> LineSource {
        LineSource { kind }
    }

    pub fn get_user_id(&self) -> String {
        match self.kind.clone() {
            LineSourceType::User { user_id } => user_id,
            _                            => String::new()
        }
    }
}