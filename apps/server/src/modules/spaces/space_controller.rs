use lunarus::prelude::*;

use crate::modules::users::model::UserModel;

use super::space_service::SpaceService;

pub async fn index(State(db): State<Db>, user: UserModel) -> Result<Response> {
    let spaces = SpaceService::new(db)
        .get_spaces(user.id.to_string())
        .await?;
    res::json(spaces)
}
