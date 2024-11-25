#[derive(Debug)]
pub enum UserSpaceRole {
    Owner,
    Editor,
    Guest,
    None,
}

impl UserSpaceRole {
    pub fn from_txt(text: &str) -> UserSpaceRole {
        match text {
            "OWNER" => UserSpaceRole::Owner,
            "EDITOR" => UserSpaceRole::Editor,
            "GUEST" => UserSpaceRole::Guest,
            _ => UserSpaceRole::None,
        }
    }
}
