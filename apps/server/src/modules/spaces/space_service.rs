use super::{
    dto::{CreateSpaceDTO, UpdateSpaceDTO},
    model::{SpaceModel, UserSpaceModel, UserSpaceRole},
};
use crate::modules::{
    spaces::serializers::get_spaces_serializer::GetSpaceSerializer, users::model::UserModel,
};
use crate::utils::hash;
use lunarus::prelude::*;
use regex::Regex;

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
        let slug = new_space.slug.unwrap_or(create_slug(&new_space.name));
        let result: Option<SpaceModel> = self
            .db
            .query("
                BEGIN;
                    LET $result = (CREATE type::table($space_table) SET name = $name, slug = $slug, description = $description, icon = $icon, primary = $primary);
                    RELATE (type::record($user))->(type::table($relation_table))->(type::record($result[0].id)) SET user_role = $role;
                    RETURN $result[0];
                COMMIT;
            ")
            .bind(("space_table", SpaceModel::TABLE_NAME))
            .bind(("relation_table", UserSpaceModel::TABLE_NAME))
            .bind(("name", new_space.name.clone()))
            .bind(("slug", slug))
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
            .update((SpaceModel::TABLE_NAME, space_id))
            .merge(space_update)
            .await?;
        space.ok_or(Error::DatabaseQueryError)
    }

    pub async fn delete_space(&self, space_id: &str) -> Result<()> {
        self.db
            .query("DELETE type::record($space)")
            .bind(("space", format!("spaces:{space_id}")))
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
                slug: None,
            },
            user.id.to_string(),
        )
        .await
    }

    pub async fn can_user_edit(&self, space_id: &str, user_id: &str) -> Result<()> {
        let role = self.get_user_role(space_id, user_id).await?;
        let can_edit = matches!(role, UserSpaceRole::Owner | UserSpaceRole::Editor);
        if can_edit {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }

    pub async fn can_user_delete(&self, space_id: &str, user_id: &str) -> Result<()> {
        let role = self.get_user_role(space_id, user_id).await?;
        if !matches!(role, UserSpaceRole::Owner) {
            return Err(Error::Unauthorized);
        }
        // checking if the space is primary
        let record: Option<Record> = self
            .db
            .query("SELECT id FROM type::record($space) WHERE primary = false;")
            .bind(("space", format!("spaces:{space_id}")))
            .await?
            .take(0)?;
        if record.is_some() {
            Ok(())
        } else { 
            Err(Error::Unauthorized)
        }
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
        Ok(UserSpaceRole::from_txt(&user_space.user_role))
    }

    pub async fn validate_slug(&self, slug: String, name: String) -> Result<()> {
        if !verify_slug_integrity(&slug, &name) {
            return Err(Error::ValidationError("slug error".to_string()));
        }
        let result: Option<Record> = self
            .db
            .query("SELECT id FROM spaces WHERE slug = $slug")
            .bind(("slug", slug))
            .await?
            .take(0)?;
        if result.is_some() {
            Err(Error::ValidationError("slug error".to_string()))
        } else {
            Ok(())
        }
    }
}

fn verify_slug_integrity(slug: &str, text: &str) -> bool {
    let text_slug = create_slug(text);
    println!("{} = {}", slug, text_slug);
    let regex = Regex::new(r"(?m)-[a-zA-Z0-9]{5}$").unwrap();
    // trimming hash
    let slug = regex.replace(slug, "");
    let text_slug = regex.replace(&text_slug, "");
    println!("{} = {}", slug, text_slug);
    slug == text_slug
}

fn create_slug(text: &str) -> String {
    let parser_rg = Regex::new("[^a-zA-Z0-9 ]+").unwrap();
    let name = parser_rg
        .replace_all(text.trim(), "")
        .to_lowercase()
        .replace(" ", "-");
    let name = if !name.is_empty() {
        name
    } else {
        "untitled".to_string()
    };
    let random_hash = hash::get_unique_random_hash(5);
    format!("{name}-{random_hash}")
}
