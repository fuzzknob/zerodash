use lunarus::prelude::*;

use crate::modules::users::model::UserModel;

use super::{
    dto::{CreateSpaceDTO, UpdateSpaceDTO},
    space_service::SpaceService,
};

pub async fn index(State(db): State<Db>, user: UserModel) -> Result<Response> {
    let spaces = SpaceService::new(db)
        .get_spaces(user.id.to_string())
        .await?;
    res::json(spaces)
}

pub async fn create(
    State(db): State<Db>,
    user: UserModel,
    Json(new_space): Json<CreateSpaceDTO>,
) -> Result<Response> {
    new_space.validate()?;
    let space = SpaceService::new(db)
        .create_space(new_space, user.id.to_string())
        .await?;
    res::builder().status(StatusCode::CREATED).json(space)
}

pub async fn update(
    Path(id): Path<String>,
    State(db): State<Db>,
    user: UserModel,
    Json(space_update): Json<UpdateSpaceDTO>,
) -> Result<Response> {
    space_update.validate()?;
    let space_service = SpaceService::new(db);
    let can_edit = space_service
        .can_user_edit(&id, &user.id.to_string())
        .await?;
    if !can_edit {
        return Err(Error::Unauthorized);
    }
    let space = space_service.update_space(&id, space_update).await?;
    res::json(space)
}

pub async fn delete(
    Path(id): Path<String>,
    State(db): State<Db>,
    user: UserModel,
) -> Result<Response> {
    let space_service = SpaceService::new(db);
    let can_delete = space_service
        .can_user_delete(&id, &user.id.to_string())
        .await?;
    if !can_delete {
        return Err(Error::Unauthorized);
    }
    space_service.delete_space(&id).await?;
    res::message("Successfully deleted space")
}
