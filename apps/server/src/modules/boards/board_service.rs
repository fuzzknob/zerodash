use lunarus::prelude::*;

use crate::modules::spaces::model::SpaceModel;

use super::{
    dto::CreateBoardDTO,
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

    pub async fn create_default_board(&self, space_id: String) -> Result<BoardModel> {
        self.create_board(CreateBoardDTO {
            name: "Personal".to_string(),
            description: None,
            icon: None,
            space: space_id,
        })
        .await
    }
}
