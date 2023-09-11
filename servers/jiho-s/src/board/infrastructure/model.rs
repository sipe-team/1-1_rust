use sea_orm::ActiveValue::Set;
use sea_orm::NotSet;
use crate::board::application::command::board_command::{CreateBoardCommand};
use crate::board::domain::board::Board;
use crate::entities::board::{ActiveModel, Model};

impl Into<Board> for Model {
    fn into(self) -> Board {
        Board {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<Board> for &Model {
    fn into(self) -> Board {
        Board {
            id: self.id,
            name: self.name.to_owned(),
        }
    }
}

impl Into<ActiveModel> for CreateBoardCommand {
    fn into(self) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            name: Set(self.name)
        }
    }
}