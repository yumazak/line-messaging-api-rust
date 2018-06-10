pub enum LineSourceType {
    User,
    Group,
    Room,
}

pub enum LineSources {
    User { id: String },
    Group{ group_id: String , user_id: String},
    Room { room_id: String, user_id: String },
}

// struct LineSource {
//     kind: LineSourceType,
// }

// impl LineSources {
//     // pub fn new(kind: LineSourceType) -> LineSource {
//     //     LineSource { kind: kind}
//     // }

//     fn create_from_object(kind: LineSourceType, id: String) -> LineSources{
//         match kind {
//             LineSourceType::User => { LineSources::User { id } },
//             LineSourceType::Group => { LineSources::Group { id } },
//             LineSourceType::Room => { LineSources::Room { id } },
//         }
//     }
// }

// struct UserSource {
//     params: SourceParams
// }

// impl Source for UserSource {
//     fn get_id(&self) -> String {
//         self.params.id.clone()
//     }
// }

// struct GroupSource {
//     params: SourceParams
// }

// impl Source for GroupSource {
//     fn get_id(&self) -> String {
//         self.params.id.clone()
//     }
// }

// struct RoomSource {
//     params: SourceParams
// }

// impl RoomSource {
//     fn get_id(&self) -> String {
//         self.params.id.clone()
//     }
// }