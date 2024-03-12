
pub struct UserEntity {
    pub name: String,
}

impl UserEntity {
    pub fn default() -> UserEntity {
        UserEntity {
            name: String::new(),
        }
    }
}
