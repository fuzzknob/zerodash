use super::{
    dto::{CreateSpaceDTO, UpdateSpaceDTO},
    model::{SpaceModel, UserSpaceModel},
};
use crate::modules::{
    spaces::serializers::get_spaces_serializer::GetSpaceSerializer, users::model::UserModel,
};
use lunarus::prelude::*;

pub struct SpaceService {
    db: Db,
}

impl SpaceService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn get_spaces(&self, user_id: String) -> Result<Vec<GetSpaceSerializer>> {
        let spaces: Vec<GetSpaceSerializer> = self
            .db
            .query("SELECT *, (SELECT * FROM boards where space = $parent.id) as boards FROM (type::record($user))->users_spaces->spaces ORDER BY created_at;")
            .bind(("user", format!("{}:{}", UserModel::TABLE_NAME, user_id)))
            .await?
            .take(0)?;
        Ok(spaces)
    }

    pub async fn create_space(
        &self,
        new_space: CreateSpaceDTO,
        user: String,
    ) -> Result<SpaceModel> {
        let result: Option<SpaceModel> = self
            .db
            .query("
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
            .bind(("user", format!("{}:{}", UserModel::TABLE_NAME, user)))
            .bind(("role", "OWNER"))
            .await?
            .take(0)?;
        Ok(result.unwrap())
    }

    pub async fn update_space(
        &self,
        space_id: &str,
        space_update: UpdateSpaceDTO,
    ) -> Result<SpaceModel> {
        let space: Option<SpaceModel> = self
            .db
            .query("UPDATE type::record($space) SET name = $name, description = $description, icon = $icon;")
            .bind(("space", format!("spaces:{space_id}")))
            .bind(("name", space_update.name))
            .bind(("description", space_update.description))
            .bind(("icon", space_update.icon))
            .await?
            .take(0)?;
        space.ok_or(Error::DatabaseQueryError)
    }

    pub async fn delete_space(&self, space_id: &str) -> Result<()> {
        let space_id = format!("spaces:{space_id}");
        self.db
            .query("DELETE type::record($space)")
            .bind(("space", space_id))
            .await?;
        Ok(())
    }

    pub async fn create_default_space(&self, user: UserModel) -> Result<SpaceModel> {
        self.create_space(
            CreateSpaceDTO {
                name: format!("{}'s space", user.first_name()),
                description: None,
                icon: Some("home".to_string()),
                primary: Some(true),
            },
            user.id.to_string(),
        )
        .await
    }

    pub async fn can_user_edit(&self, space_id: &str, user_id: &str) -> Result<bool> {
        let role = self.get_user_role(space_id, user_id).await?;
        let can_edit = matches!(role, UserSpaceRole::Owner | UserSpaceRole::Editor);
        Ok(can_edit)
    }

    pub async fn can_user_delete(&self, space_id: &str, user_id: &str) -> Result<bool> {
        let role = self.get_user_role(space_id, user_id).await?;
        if !matches!(role, UserSpaceRole::Owner) {
            return Ok(false);
        }
        // checking if the space is primary
        let record: Option<Record> = self
            .db
            .query("SELECT id FROM type::record($space) WHERE primary = false;")
            .bind(("space", format!("spaces:{space_id}")))
            .await?
            .take(0)?;
        Ok(record.is_some())
    }

    pub async fn get_user_role(&self, space_id: &str, user_id: &str) -> Result<UserSpaceRole> {
        let user_space : Option<UserSpaceModel> = self
            .db
            .query("SELECT * FROM (type::record($user))->users_spaces WHERE out = type::record($space)")
            .bind(("user", format!("users:{user_id}")))
            .bind(("space", format!("spaces:{space_id}")))
            .await?
            .take(0)?;
        let Some(user_space) = user_space else {
            return Ok(UserSpaceRole::None);
        };
        let role = match user_space.user_role.as_str() {
            "OWNER" => UserSpaceRole::Owner,
            "EDITOR" => UserSpaceRole::Editor,
            _ => UserSpaceRole::Guest,
        };
        Ok(role)
    }
}

pub enum UserSpaceRole {
    Owner,
    Editor,
    Guest,
    None,
}
