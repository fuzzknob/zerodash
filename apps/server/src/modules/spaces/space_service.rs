use super::{
    dto::CreateSpaceDTO,
    model::{SpaceModel, UserSpaceModel},
};
use crate::modules::users::model::UserModel;
use lunarus::prelude::*;

pub struct SpaceService {
    db: Db,
}

impl SpaceService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn create_space(&self, new_space: CreateSpaceDTO) -> Result<SpaceModel> {
        let result: Option<SpaceModel> = self.db.query("
            BEGIN;
                LET $result = (CREATE type::table($space_table) SET name = $name, description = $description, icon = $icon, primary = $primary);
                RELATE (type::record($user))->(type::table($relation_table))->(type::record($result[0].id)) SET user_role = $role;
                RETURN $result[0];
            COMMIT;
        ")
        .bind(("space_table", SpaceModel::TABLE_NAME))
        .bind(("relation_table", UserSpaceModel::TABLE_NAME))
        .bind(("name", new_space.name))
        .bind(("description", new_space.description))
        .bind(("icon", new_space.icon))
        .bind(("primary", new_space.primary))
        .bind(("user", format!("{}:{}", UserModel::TABLE_NAME, new_space.user)))
        .bind(("role", "OWNER"))
        .await?
        .take(0)?;
        Ok(result.unwrap())
    }

    pub async fn create_default_space(&self, user: UserModel) -> Result<SpaceModel> {
        self.create_space(CreateSpaceDTO {
            name: format!("{}'s space", user.first_name()),
            description: None,
            icon: Some("home".to_string()),
            primary: Some(true),
            user: user.id.to_string(),
        })
        .await
    }
}
