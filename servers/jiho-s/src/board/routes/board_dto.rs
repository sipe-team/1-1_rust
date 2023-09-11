use crate::board::domain::board::Board;

#[derive(serde::Deserialize)]
pub struct CreateBoardRequestDto {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct BoardResponseDto {
    pub id: i32,
    pub name: String,
}

#[derive(serde::Deserialize)]
pub struct UpdateBoardRequestDto {
    pub name: String,
}

impl From<Board> for BoardResponseDto {
    fn from(value: Board) -> Self {
        BoardResponseDto {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<&Board> for BoardResponseDto {
    fn from(value: &Board) -> Self {
        BoardResponseDto {
            id: value.id,
            name: value.name.to_owned(),
        }
    }
}