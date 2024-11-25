use crate::modules::spaces::model::{SpaceModel, UserSpaceModel, UserSpaceRole};
use lunarus::prelude::*;

use super::{
    dto::{CreateBoardDTO, UpdateBoardDto},
    model::{BoardModel, TaskStatesModel},
};

pub struct BoardService {
    db: Db,
}

impl BoardService {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn create_board(&self, new_board: CreateBoardDTO) -> Result<BoardModel> {
        let result: Option<BoardModel> = self.db.query("
            BEGIN TRANSACTION;
                LET $board = (CREATE type::table($board_table) SET name = $name, description = $description, icon = $icon, space = type::record($space))[0];
                CREATE type::table($task_states_table) SET name = 'Todo', color = '#0557FC', state_type = 'TODO', index = 0, board = type::record($board.id);
                CREATE type::table($task_states_table) SET name = 'Completed', color = '#4BB543', state_type = 'COMPLETED', index = 1, board = type::record($board.id);
                RETURN $board;
            COMMIT TRANSACTION;
        ")
        .bind(("board_table", BoardModel::TABLE_NAME))
        .bind(("name", new_board.name))
        .bind(("description", new_board.description))
        .bind(("icon", new_board.icon))
        .bind(("space", format!("{}:{}", SpaceModel::TABLE_NAME, new_board.space)))
        .bind(("task_states_table", TaskStatesModel::TABLE_NAME))
        .await?
        .take(0)?;
        Ok(result.unwrap())
    }

    pub async fn update_board(
        &self,
        board_id: String,
        board_update: UpdateBoardDto,
    ) -> Result<BoardModel> {
        let board: Option<BoardModel> = self
            .db
            .update((BoardModel::TABLE_NAME, board_id))
            .merge(board_update)
            .await?;
        board.ok_or(Error::DatabaseQueryError)
    }

    pub async fn delete_board(&self, board_id: String) -> Result<()> {
        self.db
            .query("DELETE type::record($board)")
            .bind(("board", format!("boards:{board_id}")))
            .await?;
        Ok(())
    }

    pub async fn create_default_board(&self, space_id: String) -> Result<BoardModel> {
        self.create_board(CreateBoardDTO {
            name: "Personal".to_string(),
            description: None,
            icon: None,
            space: space_id,
        })
        .await
    }

    pub async fn check_user_permission(&self, board_id: &str, user_id: &str) -> Result<()> {
        let user_space:Option<UserSpaceModel> = self.db.query(
                "SELECT * FROM (type::record($board)).space<-users_spaces WHERE in = type::record($user);"
            )
            .bind(("board", format!("boards:{board_id}")))
            .bind(("user", format!("users:{user_id}")))
            .await?
            .take(0)?;
        let Some(user_space) = user_space else {
            return Err(Error::Unauthorized);
        };
        let user_role = UserSpaceRole::from_txt(&user_space.user_role);
        if matches!(user_role, UserSpaceRole::Owner | UserSpaceRole::Editor) {
            Ok(())
        } else {
            Err(Error::Unauthorized)
        }
    }
}
