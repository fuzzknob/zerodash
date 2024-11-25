use lunarus::prelude::*;

use super::board_service::BoardService;
use super::dto::CreateBoardDTO;
use super::dto::UpdateBoardDto;
use crate::modules::spaces::space_service::SpaceService;
use crate::modules::users::model::UserModel;

pub async fn create(
    State(db): State<Db>,
    user: UserModel,
    Json(new_board): Json<CreateBoardDTO>,
) -> Result<Response> {
    new_board.validate()?;
    SpaceService::new(db.clone())
        .can_user_edit(&new_board.space, &user.id.to_string())
        .await?;
    let board = BoardService::new(db).create_board(new_board).await?;
    res::builder().status(StatusCode::CREATED).json(board)
}

pub async fn update(
    State(db): State<Db>,
    user: UserModel,
    Path(board_id): Path<String>,
    Json(board_update): Json<UpdateBoardDto>,
) -> Result<Response> {
    board_update.validate()?;
    let board_service = BoardService::new(db);
    board_service
        .check_user_permission(&board_id, &user.id.to_string())
        .await?;
    let board = board_service.update_board(board_id, board_update).await?;
    res::json(board)
}

pub async fn delete(
    State(db): State<Db>,
    user: UserModel,
    Path(board_id): Path<String>,
) -> Result<Response> {
    let board_service = BoardService::new(db);
    board_service.check_user_permission(&board_id, &user.id.to_string()).await?;
    board_service.delete_board(board_id).await?;
    res::message("Successfully deleted space")
}
