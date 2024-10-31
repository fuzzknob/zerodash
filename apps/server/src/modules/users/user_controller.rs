use lunarus::prelude::*;

use super::model::UserModel;

pub async fn me(user: UserModel) -> Result<Response> {
    res::json(user)
}
